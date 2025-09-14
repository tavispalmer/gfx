use std::{mem::{self, MaybeUninit}, ptr, rc::Rc, slice};

use crate::{gl::{self, Gl}, shader::Shader};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexPosition {
    pub position: glm::vec2,
}

impl VertexPosition {
    pub const fn new(position: glm::vec2) -> Self {
        Self {
            position,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Quad {
    pub tl: VertexPosition,
    pub tr: VertexPosition,
    pub bl: VertexPosition,
    pub br: VertexPosition,
}

impl Quad {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            tl: VertexPosition::new(glm::vec2::new(x, y)),
            tr: VertexPosition::new(glm::vec2::new(x + w, y)),
            bl: VertexPosition::new(glm::vec2::new(x, y + h)),
            br: VertexPosition::new(glm::vec2::new(x + w, y + h)),
        }
    }
}

pub struct QuadStream {
    gl: Rc<Gl>,
    shader: Shader,

    quad_vec: Vec<Quad>,
    index: Box<[u16]>,

    vertex_array: u32,
    quad_buf: u32,
    index_buf: u32,
}

impl QuadStream {
    pub fn new(gl: Rc<Gl>, shader: Shader) -> Self {
        let quad_vec = Vec::with_capacity(256);
        let mut index = Box::new_uninit_slice(6 * quad_vec.capacity());
        for i in 0..quad_vec.capacity() {
            index[i*6+0] = MaybeUninit::new((i*4+0) as u16);
            index[i*6+1] = MaybeUninit::new((i*4+1) as u16);
            index[i*6+2] = MaybeUninit::new((i*4+2) as u16);
            index[i*6+3] = MaybeUninit::new((i*4+1) as u16);
            index[i*6+4] = MaybeUninit::new((i*4+3) as u16);
            index[i*6+5] = MaybeUninit::new((i*4+2) as u16);
        }
        let index = unsafe { index.assume_init() };

        let (vertex_array, quad_buf, index_buf);
        unsafe {
            let mut vertex_arrays = [MaybeUninit::uninit(); 1];
            gl.gen_vertex_arrays(slice::from_raw_parts_mut(vertex_arrays.as_mut_ptr().cast(), vertex_arrays.len()));
            vertex_array = vertex_arrays[0].assume_init();

            let mut buffers = [MaybeUninit::uninit(); 2];
            gl.gen_buffers(slice::from_raw_parts_mut(buffers.as_mut_ptr().cast(), buffers.len()));
            quad_buf = buffers[0].assume_init();
            index_buf = buffers[1].assume_init();

            gl.bind_vertex_array(vertex_array);

            gl.bind_buffer(gl::ARRAY_BUFFER, quad_buf);
            gl.buffer_data(
                gl::ARRAY_BUFFER,
                (quad_vec.capacity() * size_of::<Quad>()).cast_signed(),
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);
            gl.buffer_data(
                gl::ELEMENT_ARRAY_BUFFER,
                (index.len() * size_of::<u16>()).cast_signed(),
                index.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            let position = gl.get_attrib_location(shader.id(), c"position").cast_unsigned();
            gl.vertex_attrib_pointer(position, 2, gl::FLOAT, false, size_of::<glm::vec2>() as i32, 0);
            gl.enable_vertex_attrib_array(position);

            gl.bind_vertex_array(0);
        }

        Self {
            gl,
            shader,
            quad_vec,
            index,
            vertex_array,
            quad_buf,
            index_buf,
        }
    }

    pub fn write(&mut self, data: &[Quad]) {
        self.quad_vec.extend(data);
    }

    pub fn flush(&mut self) {
        unsafe {
            self.gl.bind_buffer(gl::ARRAY_BUFFER, self.quad_buf);
            self.gl.buffer_sub_data(
                gl::ARRAY_BUFFER,
                0,
                (self.quad_vec.len() * size_of::<Quad>()).cast_signed(),
                self.quad_vec.as_ptr().cast(),
            );
            self.gl.bind_vertex_array(self.vertex_array);
            self.gl.use_program(self.shader.id());
            self.gl.draw_elements(gl::TRIANGLES, (self.quad_vec.len() * 6) as i32, gl::UNSIGNED_SHORT, 0);
            self.gl.bind_vertex_array(0);
        }

        self.quad_vec.clear();
    }
}
