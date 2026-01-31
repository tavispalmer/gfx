use std::{
    ffi::{CStr, c_void},
    rc::Rc,
};

use crate::{
    gl::{self, Gl},
    shader::Shader,
    sprite_batch::{Quad, QuadStream},
    tex::Texture,
};

pub struct Context {
    gl: Rc<Gl>,
    quad_stream: QuadStream,
    texture_cache: Vec<Option<Texture>>,
    current_texture: usize,
}

impl Context {
    const VERTEX_SOURCE: &str = "#version 140
uniform mat4 ortho;
in vec4 position;
in vec2 texture_coordinate;
out vec2 frag_texture_coordinate;
void main() {
    gl_Position = ortho * position;
    frag_texture_coordinate = texture_coordinate;
}";

    const FRAGMENT_SOURCE: &str = "#version 140
in vec2 frag_texture_coordinate;
out vec4 FragColor;
uniform sampler2D frag_texture;
void main() {
    FragColor = texture(frag_texture, frag_texture_coordinate);
}";

    pub fn new<F: FnMut(&CStr) -> *const c_void>(f: F) -> Self {
        let gl = Rc::new(Gl::load(f));
        let shader =
            unsafe { Shader::new(Rc::clone(&gl), Self::VERTEX_SOURCE, Self::FRAGMENT_SOURCE) };
        let quad_stream = QuadStream::new(Rc::clone(&gl), shader);
        Self {
            gl,
            quad_stream,
            texture_cache: Vec::new(),
            current_texture: 0,
        }
    }

    pub fn bind_framebuffer(&self, framebuffer: u32) {
        let gl = self.gl.as_ref();
        unsafe { gl.bind_framebuffer(gl::FRAMEBUFFER, framebuffer) }
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { self.gl.viewport(x, y, width, height) }
    }

    pub fn view(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.quad_stream
            .view(x as f32, y as f32, w as f32, h as f32)
    }

    // clear screen
    pub fn clear(&self) {
        let gl = self.gl.as_ref();
        unsafe {
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn tex_new_1bpp(&mut self, buf: &[u8], w: usize, h: usize) -> u32 {
        let texture = Texture::load_1bpp(Rc::clone(&self.gl), buf, w, h);
        // find the first empty slot
        let index;
        'outer: {
            for (t, i) in self.texture_cache.iter_mut().zip(0..) {
                if t.is_none() {
                    index = i;
                    *t = Some(texture);
                    break 'outer;
                }
            }
            index = self.texture_cache.len();
            self.texture_cache.push(Some(texture));
        }
        index.try_into().unwrap()
    }

    pub fn tex(&mut self, tex: u32) {
        if self.current_texture != tex as usize {
            self.commit();
            self.current_texture = tex as usize;
        }
    }

    pub fn sprite(&mut self, n: u32, x: i32, y: i32) {
        let tex = self.texture_cache[self.current_texture].as_ref().unwrap();
        self.quad_stream.write(&[Quad::new(
            x as f32,
            y as f32,
            tex.tile_w() as f32,
            tex.tile_h() as f32,
            (n as usize % tex.tiles_w() * tex.tile_w()) as f32 / tex.size() as f32,
            (n as usize % (tex.tiles_w() * tex.tiles_h()) / tex.tiles_w() * tex.tile_h()) as f32
                / tex.size() as f32,
            tex.tile_w() as f32 / tex.size() as f32,
            tex.tile_h() as f32 / tex.size() as f32,
        )]);
    }

    pub fn commit(&mut self) {
        self.quad_stream.flush();
    }
}
