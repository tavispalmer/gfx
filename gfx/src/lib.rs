mod color;
pub mod gl;

use std::{cell::RefCell, ffi::{c_void, CStr}, rc::Rc};

pub use color::Color;
pub use gl::GL;

use crate::gl::QuadStream;

pub trait Gfx {
    fn clear(&mut self, color: u32);
}

pub struct GfxGL {
    gl: Rc<GL>,
    framebuffer: u32,
}

impl GfxGL {
    pub fn set_framebuffer(&mut self, framebuffer: u32, width: usize, height: usize) {
        self.framebuffer = framebuffer;
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn new_quad_stream(&mut self) -> QuadStream {
        QuadStream::new(Rc::clone(&self.gl))
    }
}

impl Gfx for GfxGL {
    fn clear(&mut self, color: u32) {
        unsafe {
            self.gl.bind_framebuffer(gl::FRAMEBUFFER, self.framebuffer);
            self.gl.clear_color(
                0.3,
                0.4,
                0.5,
                1.0,
            );
            self.gl.clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
            );
        }
    }
}

// implementations
pub fn new_gl<F: FnMut(&CStr) -> *const c_void>(f: F) -> GfxGL {
    GfxGL {
        gl: Rc::new(GL::load(f)),
        framebuffer: 0,
    }
}
