// very *cool* name

use std::{alloc::{self, alloc, dealloc, realloc, GlobalAlloc, Layout}, ffi::{c_void, CStr}, io::Write, marker::PhantomData, mem::{self, MaybeUninit}, ptr::{self, NonNull}, rc::Rc, slice};

use crate::{gfx::{buffer::Buffer, Quad, Shader, Vertex, VertexFormat, VertexUsage}, gl, GL};

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
    vbo: Option<Buffer>,
    ebo: Option<Buffer>,

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
                vbo: None,
                ebo: None,
                shader,
            }
        }
    }

    fn grow(&mut self) {
        unsafe {
            // allocate new buffers
            let new_vbo = Buffer::new(self.gl.clone(), self.buf_cap * size_of::<Quad<T>>(), gl::STREAM_DRAW);
            let new_ebo = Buffer::new(self.gl.clone(), self.buf_cap * 6 * size_of::<u16>(), gl::STATIC_DRAW);

            // copy over already generated data
            if let Some(ebo) = &self.ebo {
                new_ebo.copy_from_buffer(ebo, 0, 0, ebo.size());
            }

            // pre-generate new ebo entries
            let begin = match &self.ebo {
                Some(ebo) => ebo.size() / size_of::<u16>(),
                None => 0,
            };
            let end = new_ebo.size() / size_of::<u16>();
            let mut ebo_data = Box::new_uninit_slice(end-begin);
            for i in begin/6..end/6 {
                ebo_data[(i-begin)*6+0] = MaybeUninit::new((i*4+0) as u16);
                ebo_data[(i-begin)*6+1] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[(i-begin)*6+2] = MaybeUninit::new((i*4+2) as u16);
                ebo_data[(i-begin)*6+3] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[(i-begin)*6+4] = MaybeUninit::new((i*4+3) as u16);
                ebo_data[(i-begin)*6+5] = MaybeUninit::new((i*4+2) as u16);
            }
            let ebo_data = ebo_data.assume_init();

            new_ebo.copy_from_slice(slice::from_raw_parts(
                ebo_data.as_ptr().cast(),
                ebo_data.len() / size_of::<u16>(),
            ), begin * size_of::<u16>());

            // reset vertex attributes
            self.gl.bind_vertex_array(self.vao);
            new_vbo.bind(gl::ARRAY_BUFFER);
            new_ebo.bind(gl::ELEMENT_ARRAY_BUFFER);
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

            self.vbo = Some(new_vbo);
            self.ebo = Some(new_ebo);
        }
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
        let size = match &self.vbo {
            Some(vbo) => vbo.size(),
            None => 0,
        };
        if size != self.buf_cap {
            self.grow();
        }
        unsafe {
            // copy over data
            if let Some(vbo) = &self.vbo {
                vbo.copy_from_slice(slice::from_raw_parts(
                    self.buf.as_ptr().cast(),
                    self.buf_len * size_of::<Quad<T>>(),
                ), 0);
            }

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
        }
    }
}
