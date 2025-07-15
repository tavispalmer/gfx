// very *cool* name

use std::{ffi::{c_void, CStr}, io::Write, mem::MaybeUninit, ptr::{self, NonNull}, rc::Rc};

use crate::{gfx::{Quad, Shader}, gl, GL};

pub struct QuadStream {
    // vtable
    gl: Rc<GL>,

    // gl objects
    vao: u32,

    vbo: u32,
    
    ebo: u32,

    len: usize,
    cap: usize,

    // vertex attrib location
    cur_prog: u32,
}

impl QuadStream {
    pub fn new(gl: Rc<GL>, vert: u32, tex_coord: u32) -> Self {
        unsafe {
            let mut vertex_arrays = [0; 1];
            gl.gen_vertex_arrays(&mut vertex_arrays);
            let vao = vertex_arrays[0];

            let mut quad_stream = Self {
                gl,
                vao,
                vbo: 0,
                ebo: 0,
                len: 0,
                cap: 0,
                cur_prog: 0,
            };

            // start with 256
            quad_stream.ensure_capacity(256);

            quad_stream
        }
    }

    fn ensure_capacity(&mut self, capacity: usize) {
        if capacity <= self.cap {
            return;
        }

        unsafe {
            // allocate new buffers
            let mut new_buffers = [0; 2];
            self.gl.gen_buffers(&mut new_buffers);
            let new_vbo = new_buffers[0];
            let new_ebo = new_buffers[1];

            // new vbo
            self.gl.bind_buffer(gl::COPY_WRITE_BUFFER, new_vbo);
            self.gl.buffer_data(
                gl::COPY_WRITE_BUFFER,
                (capacity * size_of::<Quad>()) as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
            if self.len != 0 {
                self.gl.bind_buffer(gl::COPY_READ_BUFFER, self.vbo);
                self.gl.copy_buffer_sub_data(
                    gl::COPY_READ_BUFFER,
                    gl::COPY_WRITE_BUFFER,
                    0,
                    0,
                    (self.len * size_of::<Quad>()) as isize,
                );
            }

            // new ebo
            self.gl.bind_buffer(gl::COPY_WRITE_BUFFER, new_ebo);
            self.gl.buffer_data(
                gl::COPY_WRITE_BUFFER,
                (capacity * 6 * size_of::<u16>()) as isize,
                ptr::null(),
                gl::STATIC_DRAW,
            );
            if self.cap != 0 {
                self.gl.bind_buffer(gl::COPY_READ_BUFFER, self.ebo);
                self.gl.copy_buffer_sub_data(
                    gl::COPY_READ_BUFFER,
                    gl::COPY_WRITE_BUFFER,
                    0,
                    0,
                    (self.cap * 6 * size_of::<u16>()) as isize,
                );
                // delete exiting arrays here
                self.gl.bind_buffer(gl::COPY_READ_BUFFER, 0);
                self.gl.delete_buffers(&[self.vbo, self.ebo]);
            }

            self.vbo = new_vbo;
            self.ebo = new_ebo;

            // pre-generate new ebo entries
            let mut ebo_data = Box::new_uninit_slice((capacity-self.cap)*6);
            for i in self.cap..capacity {
                ebo_data[i*6+0] = MaybeUninit::new((i*4+0) as u16);
                ebo_data[i*6+1] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[i*6+2] = MaybeUninit::new((i*4+2) as u16);
                ebo_data[i*6+3] = MaybeUninit::new((i*4+1) as u16);
                ebo_data[i*6+4] = MaybeUninit::new((i*4+3) as u16);
                ebo_data[i*6+5] = MaybeUninit::new((i*4+2) as u16);
            }
            let ebo_data = ebo_data.assume_init();

            // ebo is still on gl::COPY_WRITE_BUFFER
            self.gl.buffer_sub_data(
                gl::COPY_WRITE_BUFFER,
                (self.cap * 6 * size_of::<u16>()) as isize,
                (ebo_data.len() * size_of::<u16>()) as isize,
                ebo_data.as_ptr() as *const c_void,
            );
        }
        self.cap = capacity;
    }

    pub fn write(&mut self, buf: &[Quad]) {
        // write to buffer
        if buf.len() != 0 {
            // attempt resize
            let mut new_cap = self.cap;
            while buf.len() + self.len > new_cap {
                // from monogame SpriteBatcher
                new_cap += new_cap / 2;
                new_cap = (new_cap + 63) & !63;
            }
            if new_cap != self.cap {
                self.ensure_capacity(new_cap);
            }

            unsafe {
                self.gl.bind_buffer(gl::COPY_WRITE_BUFFER, self.vbo);
                self.gl.buffer_sub_data(
                    gl::COPY_WRITE_BUFFER,
                    (self.len * size_of::<Quad>()) as isize,
                    (buf.len() * size_of::<Quad>()) as isize,
                    buf.as_ptr() as *const c_void,
                );
            }
            self.len += buf.len();
        }
    }

    pub fn flush(&mut self, shader: &Shader) {
        // draw elements
        unsafe {
            // bind shader
            shader.bind();

            // reset vertex attributes
            self.gl.bind_vertex_array(self.vao);
            if shader.id() != self.cur_prog {
                self.gl.bind_buffer(gl::ARRAY_BUFFER, self.vbo);
                self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                self.gl.vertex_attrib_pointer(
                    shader.vert(),
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * size_of::<f32>()) as i32,
                    ptr::null(),
                );
                self.gl.enable_vertex_attrib_array(shader.vert());
                self.gl.vertex_attrib_pointer(
                    shader.tex_coord(),
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (4 * size_of::<f32>()) as i32,
                    (2 * size_of::<f32>()) as *const c_void,
                );
                self.gl.enable_vertex_attrib_array(shader.tex_coord());
            }

            // draw elements
            self.gl.draw_elements(
                gl::TRIANGLES,
                (self.len * 6) as i32,
                gl::UNSIGNED_SHORT,
                ptr::null(),
            );
        }

        // clear
        self.len = 0;
    }
}

impl Drop for QuadStream {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffers(&[self.vbo, self.ebo]);
            self.gl.delete_vertex_arrays(&[self.vao]);
        }
    }
}