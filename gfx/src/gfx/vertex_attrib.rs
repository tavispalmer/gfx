// inspired by mono game

pub enum VertexUsage {
    Position,
    TextureCoordinate,
}

pub enum VertexFormat {
    Vec2,
}

pub struct VertexAttrib {
    pub usage: VertexUsage,
    pub format: VertexFormat,
    pub offset: usize,
}

pub trait Vertex {
    fn vertex_attrib() -> &'static [VertexAttrib];
    // store associated shader code
    fn vertex_shader() -> &'static str;
    fn fragment_shader() -> &'static str;
}

impl VertexAttrib {
    #[inline]
    pub const fn new(usage: VertexUsage, format: VertexFormat, offset: usize) -> Self {
        Self {
            usage,
            format,
            offset,
        }
    }
}
