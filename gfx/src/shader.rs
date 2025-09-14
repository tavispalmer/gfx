use std::rc::Rc;

use crate::gl::{self, Gl};

pub struct Shader {
    gl: Rc<Gl>,
    program: u32,
}

impl Shader {
    pub unsafe fn new(gl: Rc<Gl>, vertex_source: &str, fragment_source: &str) -> Self {
        let program;
        unsafe {
            let gl = gl.as_ref();
            program = gl.create_program();
            let vertex = gl.create_shader(gl::VERTEX_SHADER);
            let fragment = gl.create_shader(gl::FRAGMENT_SHADER);

            gl.shader_source(vertex, vertex_source);
            gl.shader_source(fragment, fragment_source);
            gl.compile_shader(vertex);
            gl.compile_shader(fragment);

            gl.attach_shader(program, vertex);
            gl.attach_shader(program, fragment);
            gl.link_program(program);
            gl.delete_shader(vertex);
            gl.delete_shader(fragment);
        }

        Self {
            gl,
            program,
        }
    }

    pub fn id(&self) -> u32 {
        self.program
    }
}

impl Drop for Shader {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let gl = self.gl.as_ref();
            gl.delete_program(self.program);
        }
    }
}
