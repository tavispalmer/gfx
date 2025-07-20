use std::ffi::{c_void, CStr};

use gfx::{Color, Gfx, GfxGL, GfxSoftware, SpriteOptions, TextureGL};

pub struct App {
    gfx: Option<GfxGL>,
    tex: Option<TextureGL>,
}

impl App {
    pub const WIDTH: usize = 256;
    pub const HEIGHT: usize = 240;
    pub const ASPECT_RATIO: f32 = 4.0 / 3.0;

    pub fn new() -> Self {
        Self {
            gfx: None,
            tex: None,
        }
    }

    pub fn context_reset<F: FnMut(&CStr) -> *const c_void>(&mut self, f: F) {
        self.gfx = Some(gfx::new_gl(f));
        self.tex = Some(self.gfx.as_ref().unwrap().load_texture("awesomeface.png").unwrap());
    }

    pub fn context_destroy(&mut self) {
        self.gfx = None;
        self.tex = None;
    }

    pub fn set_framebuffer(&mut self, framebuffer: u32) {
        if let Some(gfx) = &mut self.gfx {
            gfx.set_framebuffer(framebuffer, Self::WIDTH, Self::HEIGHT);
        }
    }

    pub fn run(&mut self) {
        if let Some(gfx) = &mut self.gfx {
            gfx.clear(Color::PURPLE);
            gfx.draw(SpriteOptions::default()
                .width(256)
                .height(240)
                .tex(self.tex.clone()));
            gfx.draw(SpriteOptions::default()
                .width(128)
                .height(240)
                .tex(self.tex.clone()));
            gfx.draw(SpriteOptions::default()
                .width(8)
                .height(8)
                .tex(self.tex.clone()));
            gfx.flush();
        }
    }
}