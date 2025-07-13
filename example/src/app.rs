use std::ffi::{c_void, CStr};

use gfx::{Gfx, GfxGL};

pub struct App {
    gfx: Option<GfxGL>,
}

impl App {
    pub const WIDTH: usize = 256;
    pub const HEIGHT: usize = 240;
    pub const ASPECT_RATIO: f32 = 4.0 / 3.0;

    pub fn new() -> Self {
        Self {
            gfx: None,
        }
    }

    pub fn context_reset<F: FnMut(&CStr) -> *const c_void>(&mut self, f: F) {
        self.gfx = Some(gfx::new_gl(f));
    }

    pub fn context_destroy(&mut self) {
        self.gfx = None;
    }

    pub fn set_framebuffer(&mut self, framebuffer: u32) {
        if let Some(gfx) = &mut self.gfx {
            gfx.set_framebuffer(framebuffer, Self::WIDTH, Self::HEIGHT);
        }
    }

    pub fn run(&mut self) {
        if let Some(gfx) = &mut self.gfx {
            gfx.clear(0);
            gfx.draw(0, 0);
            gfx.flush();
        }
    }
}