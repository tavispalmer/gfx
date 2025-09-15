mod context;
mod gl;
mod shader;
mod sprite_batch;

use std::{ffi::{c_char, c_void, CStr}, mem::{self, MaybeUninit}, ptr::{self, NonNull}};

use crate::{context::Context, gl::Gl, shader::Shader};

pub struct Gfx {
    context: Option<Context>,
}

impl Gfx {
    pub fn new() -> Self {
        Self {
            context: None,
        }
    }

    pub fn context_reset<F: FnMut(&CStr) -> *const c_void>(&mut self, f: F) {
        self.context = Some(Context::new(f))
    }
    
    pub fn context_destroy(&mut self) {
        self.context = None;
    }

    pub fn bind_framebuffer(&self, framebuffer: u32) {
        self.context.as_ref().unwrap().bind_framebuffer(framebuffer)
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.context.as_ref().unwrap().viewport(x, y, width, height)
    }

    pub fn view(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.context.as_mut().unwrap().view(x, y, w, h)
    }

    pub fn clear(&self) {
        self.context.as_ref().unwrap().clear()
    }

    pub fn sprite(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.context.as_mut().unwrap().sprite(x, y, w, h)
    }

    pub fn commit(&mut self) {
        self.context.as_mut().unwrap().commit()
    }
}

// c++ style init
#[unsafe(no_mangle)]
pub extern "C" fn gfx_init(gfx: &mut MaybeUninit<Gfx>) -> &mut Gfx {
    gfx.write(Gfx::new());
    unsafe {
        gfx.assume_init_mut()
    }
}

// c++ style deinit
#[unsafe(no_mangle)]
pub extern "C" fn gfx_deinit(gfx: &mut Gfx) {
    unsafe {
        ptr::drop_in_place(gfx)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_new() -> Box<Gfx> {
    Box::new(Gfx::new())
}

#[allow(unused_variables)]
#[unsafe(no_mangle)]
pub extern "C" fn gfx_delete(gfx: Box<Gfx>) {}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_context_reset(gfx: &mut Gfx, f: unsafe extern "C" fn(*const c_char) -> *const c_void) {
    gfx.context_reset(|s| unsafe { f(s.as_ptr()) })
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_context_destroy(gfx: &mut Gfx) {
    gfx.context_destroy()
}

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
pub extern "C" fn gfx_sprite(gfx: &mut Gfx, x: i32, y: i32, w: i32, h: i32) {
    gfx.sprite(x, y, w, h)
}

#[unsafe(no_mangle)]
pub extern "C" fn gfx_commit(gfx: &mut Gfx) {
    gfx.commit()
}
