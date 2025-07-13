mod gfx {
    mod quad;
    mod quad_stream;

    pub use quad::Quad;
    pub use quad_stream::QuadStream;
}

use gfx::Quad;
use gfx::QuadStream;
use glm::vec2;

mod color;
pub mod gl;

use std::{ffi::{c_void, CStr}, rc::Rc};

pub use color::Color;
pub use gl::GL;

pub trait Gfx {
    fn clear(&mut self, color: u32);
    fn draw(&mut self, x: usize, y: usize);
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
            self.gl.viewport(0, 0, width as i32, height as i32);
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

    fn draw(&mut self, x: usize, y: usize) {
        self.quad_stream.write(&[Quad::new(
            vec2::new(x as f32, y as f32),
            vec2::new((x + 1) as f32, y as f32),
            vec2::new(x as f32, (y + 1) as f32),
            vec2::new((x + 1) as f32, (y + 1) as f32)),
        ]);
    }

    fn flush(&mut self) {
        unsafe {
            self.gl.use_program(self.prog);
            self.quad_stream.flush();
        }
    }
}

// implementations
pub fn new_gl<F: FnMut(&CStr) -> *const c_void>(f: F) -> GfxGL {
        const VERTEX_SHADER: &str =
"#version 140
in vec2 vert;
void main() {
    gl_Position = vec4(vert, 0.0, 1.0);
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
