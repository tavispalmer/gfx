use std::{
    ffi::CStr,
    mem::{self, MaybeUninit},
    ptr,
    rc::Rc,
    slice,
};

use crate::{
    gl::{self, Gl},
    shader::Shader,
};

pub enum VertexElementFormat {
    Single,
    Vector2,
    Vector3,
    Vector4,
}

impl VertexElementFormat {
    pub const fn size(&self) -> usize {
        match self {
            Self::Single => 1,
            Self::Vector2 => 2,
            Self::Vector3 => 3,
            Self::Vector4 => 4,
        }
    }
}

pub enum VertexElementUsage {
    Position,
    TextureCoordinate,
    Palette,
}

impl VertexElementUsage {
    pub const fn name(&self) -> &'static CStr {
        match self {
            Self::Position => c"position",
            Self::TextureCoordinate => c"texture_coordinate",
            Self::Palette => c"palette",
        }
    }
}

pub struct VertexElement {
    pub offset: usize,
    pub format: VertexElementFormat,
    pub usage: VertexElementUsage,
}

impl VertexElement {
    pub const fn new(
        offset: usize,
        format: VertexElementFormat,
        usage: VertexElementUsage,
    ) -> Self {
        Self {
            offset,
            format,
            usage,
        }
    }
}

trait Vertex {
    fn elements() -> &'static [VertexElement];
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexPosition {
    pub position: glm::vec2,
}

impl VertexPosition {
    pub const fn new(position: glm::vec2) -> Self {
        Self { position }
    }
}

impl Vertex for VertexPosition {
    fn elements() -> &'static [VertexElement] {
        const ELEMENTS: [VertexElement; 1] = [VertexElement::new(
            mem::offset_of!(VertexPosition, position),
            VertexElementFormat::Vector2,
            VertexElementUsage::Position,
        )];
        &ELEMENTS
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexPositionTexture {
    pub position: glm::vec2,
    pub texture: glm::vec2,
}

impl VertexPositionTexture {
    pub const fn new(position: glm::vec2, texture: glm::vec2) -> Self {
        Self { position, texture }
    }
}

impl Vertex for VertexPositionTexture {
    fn elements() -> &'static [VertexElement] {
        const ELEMENTS: [VertexElement; 2] = [
            VertexElement::new(
                mem::offset_of!(VertexPositionTexture, position),
                VertexElementFormat::Vector2,
                VertexElementUsage::Position,
            ),
            VertexElement::new(
                mem::offset_of!(VertexPositionTexture, texture),
                VertexElementFormat::Vector2,
                VertexElementUsage::TextureCoordinate,
            ),
        ];
        &ELEMENTS
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Quad {
    pub tl: VertexPositionTexture,
    pub tr: VertexPositionTexture,
    pub bl: VertexPositionTexture,
    pub br: VertexPositionTexture,
}

impl Quad {
    pub const fn new(x: f32, y: f32, w: f32, h: f32, tx: f32, ty: f32, tw: f32, th: f32) -> Self {
        Self {
            tl: VertexPositionTexture::new(glm::vec2::new(x, y), glm::vec2::new(tx, ty)),
            tr: VertexPositionTexture::new(glm::vec2::new(x + w, y), glm::vec2::new(tx + tw, ty)),
            bl: VertexPositionTexture::new(glm::vec2::new(x, y + h), glm::vec2::new(tx, ty + th)),
            br: VertexPositionTexture::new(
                glm::vec2::new(x + w, y + h),
                glm::vec2::new(tx + tw, ty + th),
            ),
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
    buf_cap: usize,

    ortho: u32,
}

impl QuadStream {
    pub fn new(gl: Rc<Gl>, shader: Shader) -> Self {
        let quad_vec = Vec::with_capacity(256);
        let index = Vec::new().into_boxed_slice();

        let (vertex_array, quad_buf, index_buf, ortho);
        unsafe {
            let mut vertex_arrays = [MaybeUninit::uninit(); 1];
            gl.gen_vertex_arrays(slice::from_raw_parts_mut(
                vertex_arrays.as_mut_ptr().cast(),
                vertex_arrays.len(),
            ));
            vertex_array = vertex_arrays[0].assume_init();

            let mut buffers = [MaybeUninit::uninit(); 2];
            gl.gen_buffers(slice::from_raw_parts_mut(
                buffers.as_mut_ptr().cast(),
                buffers.len(),
            ));
            quad_buf = buffers[0].assume_init();
            index_buf = buffers[1].assume_init();

            gl.bind_vertex_array(vertex_array);
            gl.bind_buffer(gl::ARRAY_BUFFER, quad_buf);
            gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, index_buf);

            for attrib in VertexPositionTexture::elements() {
                let index = gl
                    .get_attrib_location(shader.id(), attrib.usage.name())
                    .cast_unsigned();
                gl.vertex_attrib_pointer(
                    index,
                    attrib.format.size() as i32,
                    gl::FLOAT,
                    false,
                    size_of::<VertexPositionTexture>() as i32,
                    attrib.offset,
                );
                gl.enable_vertex_attrib_array(index);
            }

            gl.bind_vertex_array(0);

            ortho = gl
                .get_uniform_location(shader.id(), c"ortho")
                .cast_unsigned();
        }

        Self {
            gl,
            shader,
            quad_vec,
            index,
            vertex_array,
            quad_buf,
            index_buf,
            buf_cap: 0,
            ortho,
        }
    }

    pub fn view(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let ortho = glm::mat4::<f32>::ortho(x, x + w, y + h, y);
        unsafe {
            self.gl.use_program(self.shader.id());
            self.gl
                .uniform_matrix4fv(self.ortho.cast_signed(), 1, false, ortho.as_ptr().cast());
        }
    }

    pub fn write(&mut self, data: &[Quad]) {
        self.quad_vec.extend(data);
    }

    pub fn flush(&mut self) {
        // resize
        if self.quad_vec.capacity() > self.buf_cap {
            self.buf_cap = self.quad_vec.capacity();
            let mut index = Box::new_uninit_slice(6 * self.buf_cap);
            for i in 0..self.buf_cap {
                index[i * 6 + 0] = MaybeUninit::new((i * 4 + 0) as u16);
                index[i * 6 + 1] = MaybeUninit::new((i * 4 + 1) as u16);
                index[i * 6 + 2] = MaybeUninit::new((i * 4 + 2) as u16);
                index[i * 6 + 3] = MaybeUninit::new((i * 4 + 1) as u16);
                index[i * 6 + 4] = MaybeUninit::new((i * 4 + 3) as u16);
                index[i * 6 + 5] = MaybeUninit::new((i * 4 + 2) as u16);
            }
            self.index = unsafe { index.assume_init() };

            unsafe {
                self.gl.bind_buffer(gl::ARRAY_BUFFER, self.quad_buf);
                self.gl.buffer_data(
                    gl::ARRAY_BUFFER,
                    (self.buf_cap * size_of::<Quad>()).cast_signed(),
                    ptr::null(),
                    gl::DYNAMIC_DRAW,
                );

                self.gl
                    .bind_buffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buf);
                self.gl.buffer_data(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (self.index.len() * size_of::<u16>()).cast_signed(),
                    self.index.as_ptr().cast(),
                    gl::STATIC_DRAW,
                );
            }
        }

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
            self.gl.draw_elements(
                gl::TRIANGLES,
                (self.quad_vec.len() * 6) as i32,
                gl::UNSIGNED_SHORT,
                0,
            );
            self.gl.bind_vertex_array(0);
        }

        self.quad_vec.clear();
    }
}
