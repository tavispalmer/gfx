use std::{mem::MaybeUninit, rc::Rc};

use glm::mat4;

use crate::{gl, GL};

pub struct Shader {
    gl: Rc<GL>,
    prog: u32,

    // attrib locations
    pos: u32,
    tex: u32,
    mat: u32,
}

impl Shader {
    pub fn new(gl: Rc<GL>, vertex_shader: &str, fragment_shader: &str) -> Self {
        unsafe {
            // compile shader for QuadStream
            let prog = gl.create_program();
            let vert = gl.create_shader(gl::VERTEX_SHADER);
            let frag = gl.create_shader(gl::FRAGMENT_SHADER);

            gl.shader_source(vert, vertex_shader);
            gl.shader_source(frag, fragment_shader);
            gl.compile_shader(vert);
            gl.compile_shader(frag);

            gl.attach_shader(prog, vert);
            gl.attach_shader(prog, frag);
            gl.link_program(prog);
            gl.delete_shader(vert);
            gl.delete_shader(frag);

            // get attrib locations
            let pos = gl.get_attrib_location(prog, c"pos") as u32;
            let tex = gl.get_attrib_location(prog, c"tex") as u32;
            let mat = gl.get_uniform_location(prog, c"mat") as u32;

            let shader = Self {
                gl,
                prog,
                pos,
                tex,
                mat,
            };

            // use identity for now
            shader.mat(&mat4::default());

            shader
        }
    }

    pub fn bind(&self) {
        unsafe {
            let mut current_program = MaybeUninit::uninit();
            self.gl.get_integerv(gl::CURRENT_PROGRAM, current_program.as_mut_ptr());
            let current_program = current_program.assume_init() as u32;

            if current_program != self.prog {
                self.gl.use_program(self.prog);
            }
        }
    }

    pub const fn pos(&self) -> u32 {
        self.pos
    }

    pub const fn tex(&self) -> u32 {
        self.tex
    }

    pub fn mat(&self, mat: &glm::mat4) {
        self.bind();
        unsafe {
            self.gl.uniform_matrix4fv(
                self.mat as i32,
                1,
                gl::FALSE,
                mat.as_ptr() as *const f32,
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.prog);
        }
    }
}
