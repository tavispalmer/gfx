use gl::Quad;
use gl::QuadStream;
use glm::mat4;
use glm::vec2;

mod color;
mod gl;
mod sprite_options;
mod texture;

use std::mem::MaybeUninit;
use std::path::Path;
use std::{ffi::{c_void, CStr}, rc::Rc};

pub use color::Color;
pub use gl::GL;
pub use sprite_options::SpriteOptions;
pub use texture::TextureGL;

use crate::gl::Pos;
use crate::gl::PosTex;

pub trait Gfx {
    fn clear(&mut self, color: Color);
    fn draw(&mut self, sprite_options: &SpriteOptions);
    fn flush(&mut self);
}

pub struct GfxSoftware<'a> {
    data: &'a mut [Color],
    width: usize,
    height: usize,
    pitch: usize,
}

pub fn new(data: &mut [Color], width: usize, height: usize, pitch: usize) -> GfxSoftware<'_> {
    GfxSoftware {
        data,
        width,
        height,
        pitch,
    }
}

impl Gfx for GfxSoftware<'_> {
    fn clear(&mut self, color: Color) {
        let width = self.width * size_of::<Color>();
        if self.pitch == width {
            self.data.fill(color);
        } else {
            for line in 0..self.height {
                let begin = line * self.pitch;
                let end = begin + width;
                self.data[begin..end].fill(color);
            }
        }
    }

    fn draw(&mut self, sprite_options: &SpriteOptions) {
        
    }

    fn flush(&mut self) {
        
    }
}

pub struct GfxGL {
    // vtable
    gl: Rc<GL>,
    // for drawing quads
    pos_quad_stream: QuadStream<Pos>,
    quad_stream: QuadStream<PosTex>,
    cur_tex: Option<TextureGL>,
    framebuffer: u32,
}

impl GfxGL {
    pub fn set_framebuffer(&mut self, framebuffer: u32, width: usize, height: usize) {
        self.framebuffer = framebuffer;
        unsafe {
            // set viewport
            self.gl.viewport(0, 0, width as i32, height as i32);

            // reset transformation matrix
            let ortho = mat4::ortho(0.0, width as f32, height as f32, 0.0);
            self.pos_quad_stream.shader().mat(&ortho);
            self.quad_stream.shader().mat(&ortho);
        }
    }

    pub fn load_texture<P: AsRef<Path>>(&self, path: P) -> Result<TextureGL, std::io::Error> {
        TextureGL::load(Rc::clone(&self.gl), path)
    }
}

impl Gfx for GfxGL {
    fn clear(&mut self, color: Color) {
        unsafe {
            self.gl.bind_framebuffer(gl::FRAMEBUFFER, self.framebuffer);
            self.gl.clear_color(
                color.r() as f32 / 255.0,
                color.g() as f32 / 255.0,
                color.b() as f32 / 255.0,
                color.a() as f32 / 255.0,
            );
            self.gl.clear(
                gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT,
            );
        }
    }

    fn draw(&mut self, sprite_options: &SpriteOptions) {
        // check texture
        if let Some(tex) = &sprite_options.tex {
            if let Some(cur_tex) = &self.cur_tex {
                if tex.id() != cur_tex.id() {
                    // flush
                    cur_tex.bind();
                    self.quad_stream.flush();
                    self.cur_tex = Some(tex.clone());
                }
            } else {
                self.cur_tex = Some(tex.clone());
            }
        }
        // gen points
        let left = sprite_options.x as f32;
        let right = (sprite_options.x + sprite_options.width) as f32;
        let top = sprite_options.y as f32;
        let bottom = (sprite_options.y + sprite_options.height) as f32;
        if let Some(_) = &sprite_options.tex {
            self.quad_stream.write(&[Quad::new(
                PosTex::new(vec2::new(left, top), vec2::new(0.0, 0.0)),
                PosTex::new(vec2::new(right, top), vec2::new(1.0, 0.0)),
                PosTex::new(vec2::new(left, bottom), vec2::new(0.0, 1.0)),
                PosTex::new(vec2::new(right, bottom), vec2::new(1.0, 1.0)),
            )]);
        } else {
            self.pos_quad_stream.write(&[Quad::new(
                Pos::new(vec2::new(left, top)),
                Pos::new(vec2::new(right, top)),
                Pos::new(vec2::new(left, bottom)),
                Pos::new(vec2::new(right, bottom)),
            )]);
        }
    }

    fn flush(&mut self) {
        unsafe {
            self.gl.bind_framebuffer(gl::FRAMEBUFFER, self.framebuffer);
        }
        if let Some(cur_tex) = &self.cur_tex {
            cur_tex.bind();
            self.quad_stream.flush();
        }
        self.pos_quad_stream.flush();
    }
}

// implementations
pub fn new_gl<F: FnMut(&CStr) -> *const c_void>(f: F) -> GfxGL {
    let gl = Rc::new(GL::load(f));

    let pos_quad_stream = QuadStream::new(Rc::clone(&gl));
    let quad_stream = QuadStream::new(Rc::clone(&gl));

    GfxGL {
        gl,
        pos_quad_stream,
        quad_stream,
        cur_tex: None,
        framebuffer: 0,
    }
}
