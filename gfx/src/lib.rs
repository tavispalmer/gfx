mod buffer;
mod context;
mod gl;
mod shader;
mod sprite_batch;
mod tex;

use std::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
    ptr,
};

use crate::{context::Context, tex::Texture};

pub struct Gfx {
    context: Context,
}

impl Gfx {
    pub fn new<F: FnMut(&CStr) -> *const c_void>(f: F) -> Self {
        Self {
            context: Context::new(f),
        }
    }

    pub fn bind_framebuffer(&self, framebuffer: u32) {
        self.context.bind_framebuffer(framebuffer)
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.context.viewport(x, y, width, height)
    }

    pub fn view(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.context.view(x, y, w, h)
    }

    pub fn clear(&self) {
        self.context.clear()
    }

    pub fn tex_new_1bpp(&mut self, buf: &[u8], w: usize, h: usize) -> u32 {
        self.context.tex_new_1bpp(buf, w, h)
    }

    pub fn sprite(&mut self, n: u32, x: i32, y: i32) {
        self.context.sprite(n, x, y)
    }

    pub fn commit(&mut self) {
        self.context.commit()
    }
}

// c++ style init
#[unsafe(no_mangle)]
pub extern "C" fn gfx_init(
    gfx: &mut MaybeUninit<Gfx>,
    f: unsafe extern "C" fn(*const c_char) -> *const c_void,
) -> &mut Gfx {
    gfx.write(Gfx::new(|s| unsafe { f(s.as_ptr()) }));
    unsafe { gfx.assume_init_mut() }
}

// c++ style deinit
#[unsafe(no_mangle)]
pub extern "C" fn gfx_deinit(gfx: &mut Gfx) {
    unsafe { ptr::drop_in_place(gfx) }
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_new(f: unsafe extern "C" fn(*const c_char) -> *const c_void) -> Box<Gfx> {
    Box::new(Gfx::new(|s| unsafe { f(s.as_ptr()) }))
}

#[allow(unused_variables)]
#[unsafe(no_mangle)]
pub extern "C" fn gfx_delete(gfx: Box<Gfx>) {}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_bind_framebuffer(gfx: &Gfx, framebuffer: u32) {
    gfx.bind_framebuffer(framebuffer)
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_viewport(gfx: &Gfx, x: i32, y: i32, width: i32, height: i32) {
    gfx.viewport(x, y, width, height)
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_view(gfx: &mut Gfx, x: i32, y: i32, w: i32, h: i32) {
    gfx.view(x, y, w, h)
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_clear(gfx: &Gfx) {
    gfx.clear()
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_sprite(gfx: &mut Gfx, n: u32, x: i32, y: i32) {
    gfx.sprite(n, x, y)
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_commit(gfx: &mut Gfx) {
    gfx.commit()
}
