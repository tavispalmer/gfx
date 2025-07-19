// very *cool* name

use std::{alloc::{self, alloc, dealloc, realloc, GlobalAlloc, Layout}, ffi::{c_void, CStr}, io::Write, marker::PhantomData, mem::MaybeUninit, ptr::{self, NonNull}, rc::Rc};

use crate::{gfx::{Quad, Shader, Vertex, VertexFormat, VertexUsage}, gl, GL};

pub struct QuadStream<T: Vertex> {
    // vtable
    gl: Rc<GL>,
    
    // buffer
    // this implementation is based on BufWriter
    buf: NonNull<Quad<T>>,
    buf_cap: usize,
    buf_len: usize,

    // gl objects
    vao: u32,

    vbo: u32,
    
    ebo: u32,

    // vec implementation on the gpu side
    cap: usize,

    shader: Shader,
}

impl<T: Vertex> QuadStream<T> {
    pub fn new(gl: Rc<GL>) -> Self {
        unsafe {
            // compile shader
            let shader = Shader::new(
                Rc::clone(&gl),
                T::vertex_shader(),
                T::fragment_shader(),
            );

            let mut vertex_arrays = [0; 1];
            gl.gen_vertex_arrays(&mut vertex_arrays);
            let vao = vertex_arrays[0];

            Self {
                gl,
                buf:
                    NonNull::new(alloc(
                        Layout::array::<Quad<T>>(256).unwrap(),
                    ).cast()).unwrap(),
                buf_cap: 256,
                buf_len: 0,
                vao,
                vbo: 0,
                ebo: 0,
                cap: 0,
                shader,
            }
        }
    }

    fn grow(&mut self) {
        unsafe {
            // allocate new buffers
            let mut new_buffers = [0; 2];
            self.gl.gen_buffers(&mut new_buffers);
            let new_vbo = new_buffers[0];
            let new_ebo = new_buffers[1];

            // new vbo
            self.gl.bind_buffer(gl::ARRAY_BUFFER, new_vbo);
            self.gl.buffer_data(
                gl::ARRAY_BUFFER,
                (self.buf_cap * size_of::<Quad<T>>()) as isize,
                ptr::null(),
                gl::STREAM_DRAW,
            );

            // new ebo
            self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, new_ebo);
            self.gl.buffer_data(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.buf_cap * 6 * size_of::<u16>()) as isize,
                ptr::null(),
                gl::STATIC_DRAW,
            );
            if self.cap != 0 {
                // copy over already generated data
                self.gl.bind_buffer(gl::COPY_READ_BUFFER, self.ebo);
                self.gl.copy_buffer_sub_data(
                    gl::COPY_READ_BUFFER,
                    gl::ELEMENT_ARRAY_BUFFER,
                    0,
                    0,
                    (self.cap * 6 * size_of::<u16>()) as isize,
                );
                // delete exiting arrays here
                self.gl.delete_buffers(&[self.vbo, self.ebo]);
            }

            self.vbo = new_vbo;
            self.ebo = new_ebo;

            // pre-generate new ebo entries
            let mut ebo_data = Box::new_uninit_slice((self.buf_cap-self.cap)*6);
            for i in self.cap..self.buf_cap {
                ebo_data[i*6+0] = MaybeUninit::new((i*4+0) as u16);
                ebo_data[i*6+1] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[i*6+2] = MaybeUninit::new((i*4+2) as u16);
                ebo_data[i*6+3] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[i*6+4] = MaybeUninit::new((i*4+3) as u16);
                ebo_data[i*6+5] = MaybeUninit::new((i*4+2) as u16);
            }
            let ebo_data = ebo_data.assume_init();

            // ebo is still on gl::ELEMENT_ARRAY_BUFFER
            self.gl.buffer_sub_data(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.cap * 6 * size_of::<u16>()) as isize,
                (ebo_data.len() * size_of::<u16>()) as isize,
                ebo_data.as_ptr().cast(),
            );

            // reset vertex attributes
            self.gl.bind_vertex_array(self.vao);
            self.gl.bind_buffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            for vertex_attrib in T::vertex_attrib() {
                let index = match vertex_attrib.usage {
                    VertexUsage::Position => self.shader.pos(),
                    VertexUsage::TextureCoordinate => self.shader.tex(),
                };
                let (size, type_) = match vertex_attrib.format {
                    VertexFormat::Vec2 => (2, gl::FLOAT),
                };
                self.gl.vertex_attrib_pointer(
                    index,
                    size,
                    type_,
                    gl::FALSE,
                    size_of::<T>() as i32,
                    vertex_attrib.offset as _,
                );
                self.gl.enable_vertex_attrib_array(index);
            }
            self.gl.bind_vertex_array(0);
        }
        self.cap = self.buf_cap;
    }

    #[inline]
    unsafe fn write_to_buffer_unchecked(&mut self, buf: &[Quad<T>]) {
        let count = buf.len();
        let src = buf.as_ptr();
        unsafe {
            let dst = self.buf.as_ptr().add(self.buf_len);
            ptr::copy_nonoverlapping(src, dst, count);
        }
        self.buf_len += count;
    }

    #[inline]
    fn spare_capacity(&self) -> usize {
        self.buf_cap - self.buf_len
    }

    pub fn write(&mut self, buf: &[Quad<T>]) {
        // write to buffer
        if buf.len() > self.spare_capacity() {
            // allocate more space
            let mut new_cap = self.buf_cap;
            while buf.len() > new_cap - self.buf_len {
                // growth function from monogame
                new_cap += new_cap / 2;
                new_cap = (new_cap + 63) & !63;
            }

            unsafe {
                self.buf = NonNull::new(realloc(
                    self.buf.as_ptr().cast(),
                    Layout::array::<Quad<T>>(self.buf_cap).unwrap(),
                    new_cap * size_of::<Quad<T>>(),
                ).cast()).unwrap();
                self.buf_cap = new_cap;
            }
        }

        unsafe {
            self.write_to_buffer_unchecked(buf);
        }
    }

    pub fn flush(&mut self) {
        // do we need to allocate more space on the gpu?
        if self.cap != self.buf_cap {
            self.grow();
        }
        unsafe {
            // copy over data
            self.gl.bind_buffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.buffer_sub_data(
                gl::ARRAY_BUFFER,
                0,
                (self.buf_len * size_of::<Quad<T>>()) as isize,
                self.buf.as_ptr().cast(),
            );

            // draw elements
            // bind shader
            self.shader.bind();

            // bind vertex array
            self.gl.bind_vertex_array(self.vao);

            // draw elements
            self.gl.draw_elements(
                gl::TRIANGLES,
                (self.buf_len * 6) as i32,
                gl::UNSIGNED_SHORT,
                ptr::null(),
            );

            // if we kept this bound, other code that binds/unbinds
            // buffers could mess with the internal state
            self.gl.bind_vertex_array(0);
        }

        // clear
        self.buf_len = 0;
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }
}

impl<T: Vertex> Drop for QuadStream<T> {
    fn drop(&mut self) {
        unsafe {
            dealloc(
                self.buf.as_ptr().cast(),
                Layout::array::<Quad<T>>(self.buf_cap).unwrap(),
            );
            self.gl.delete_vertex_arrays(&[self.vao]);
            // these are allocated on demand
            if self.vbo != 0 {
                self.gl.delete_buffers(&[self.vbo, self.ebo]);
            }
        }
    }
}
