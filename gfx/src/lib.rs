mod gfx {
    mod quad;
    mod quad_stream;

    pub use quad::Quad;
    pub use quad_stream::QuadStream;
}

use gfx::Quad;
use gfx::QuadStream;
use glm::mat4;
use glm::vec2;

mod color;
pub mod gl;
mod sprite_options;
mod texture;

use std::{ffi::{c_void, CStr}, rc::Rc};

pub use color::Color;
pub use gl::GL;
pub use sprite_options::SpriteOptions;

pub trait Gfx {
    fn clear(&mut self, color: u32);
    fn draw(&mut self, sprite_options: SpriteOptions);
    fn flush(&mut self);
}

pub struct GfxGL {
    // vtable
    gl: Rc<GL>,
    // for drawing quads
    quad_stream: QuadStream,
    prog: u32,
    mat: u32,
    framebuffer: u32,
}

impl GfxGL {
    pub fn set_framebuffer(&mut self, framebuffer: u32, width: usize, height: usize) {
        self.framebuffer = framebuffer;
        unsafe {
            // set viewport
            self.gl.viewport(0, 0, width as i32, height as i32);

            // reset transformation matrix
            let value = mat4::ortho(0.0, width as f32, height as f32, 0.0);
            self.gl.use_program(self.prog);
            self.gl.uniform_matrix4fv(
                self.mat as i32,
                1,
                gl::FALSE,
                value.as_ptr() as *const f32,
            );
            self.gl.use_program(0);
        }
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

    fn draw(&mut self, sprite_options: SpriteOptions) {
        // gen points
        let left = sprite_options.x as f32;
        let right = (sprite_options.x + sprite_options.width) as f32;
        let top = sprite_options.y as f32;
        let bottom = (sprite_options.y + sprite_options.height) as f32;
        self.quad_stream.write(&[Quad::new(
            vec2::new(left, top),
            vec2::new(right, top),
            vec2::new(left, bottom),
            vec2::new(right, bottom),
        )]);
    }

    fn flush(&mut self) {
        unsafe {
            self.gl.use_program(self.prog);
            self.quad_stream.flush();
            self.gl.use_program(0);
        }
    }
}

// implementations
pub fn new_gl<F: FnMut(&CStr) -> *const c_void>(f: F) -> GfxGL {
        const VERTEX_SHADER: &str =
"#version 140
uniform mat4 mat;
in vec2 vert;
void main() {
    gl_Position = mat * vec4(vert, 0.0, 1.0);
}";
        const FRAGMENT_SHADER: &str =
"#version 140
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0);
}";

    let gl = Rc::new(GL::load(f));

    unsafe {
        // compile shader for QuadStream
        let prog = gl.create_program();
        let vert = gl.create_shader(gl::VERTEX_SHADER);
        let frag = gl.create_shader(gl::FRAGMENT_SHADER);

        gl.shader_source(vert, VERTEX_SHADER);
        gl.shader_source(frag, FRAGMENT_SHADER);
        gl.compile_shader(vert);
        gl.compile_shader(frag);

        gl.attach_shader(prog, vert);
        gl.attach_shader(prog, frag);
        gl.link_program(prog);
        gl.delete_shader(vert);
        gl.delete_shader(frag);

        // get attrib locations
        let vert = gl.get_attrib_location(prog, c"vert") as u32;
        let mat = gl.get_uniform_location(prog, c"mat") as u32;

        // use identity for now
        gl.use_program(prog);
        let value = mat4::default();
        gl.uniform_matrix4fv(
            mat as i32,
            1,
            gl::FALSE,
            value.as_ptr() as *const f32,
        );
        gl.use_program(0);

        // quad stream
        let quad_stream = QuadStream::new(Rc::clone(&gl), vert);

        GfxGL {
            gl,
            quad_stream,
            prog,
            mat,
            framebuffer: 0,
        }
    }
}

impl Drop for GfxGL {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.prog);
        }
    }
}
