use std::{ffi::{c_void, CStr}, mem, ptr::NonNull, rc::Rc};

use crate::{gl::{self, Gl}, shader::Shader, sprite_batch::{Quad, QuadStream}};

pub struct Context {
    gl: Rc<Gl>,
    quad_stream: QuadStream,
}

impl Context {
    const VERTEX_SOURCE: &str =
"#version 140
in vec4 position;
void main() {
    gl_Position = position;
}";

    const FRAGMENT_SOURCE: &str =
"#version 140
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}";

    pub fn new<F: FnMut(&CStr) -> *const c_void>(f: F) -> Self {
        let gl = Rc::new(Gl::load(f));
        let shader = unsafe {
            Shader::new(
                Rc::clone(&gl),
                Self::VERTEX_SOURCE,
                Self::FRAGMENT_SOURCE,
            )
        };
        let quad_stream = QuadStream::new(Rc::clone(&gl), shader);
        Self {
            gl,
            quad_stream,
        }
    }

    pub fn bind_framebuffer(&self, framebuffer: u32) {
        let gl = self.gl.as_ref();
        unsafe {
            gl.bind_framebuffer(gl::FRAMEBUFFER, framebuffer)
        }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            self.gl.viewport(x, y, width, height)
        }
    }

    // clear screen
    pub fn clear(&self) {
        let gl = self.gl.as_ref();
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn sprite(&mut self, x: f32, y: f32) {
        self.quad_stream.write(&[Quad::new(x as f32, y as f32, 1.0 , 1.0)]);
    }

    pub fn commit(&mut self) {
        self.quad_stream.flush();
    }
}
