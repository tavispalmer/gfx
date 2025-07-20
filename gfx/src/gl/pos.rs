use crate::gl::{Vertex, VertexAttrib, VertexFormat, VertexUsage};

#[repr(C)]
pub struct Pos {
    pub pos: glm::vec2,
}

impl Pos {
    #[inline]
    pub const fn new(pos: glm::vec2) -> Self {
        Self {
            pos,
        }
    }
}

impl Vertex for Pos {
    fn vertex_attrib() -> &'static [VertexAttrib] {
        const VERTEX_ATTRIB: [VertexAttrib; 1] = [
            VertexAttrib::new(
                VertexUsage::Position,
                VertexFormat::Vec2,
                0,
            ),
        ];
        &VERTEX_ATTRIB
    }

    fn vertex_shader() -> &'static str {
        concat!(
            "#version 140\n",
            "uniform mat4 mat;",
            "in vec2 pos;",
            "void main() {",
                "gl_Position = mat * vec4(pos, 0.0, 1.0);",
            "}",
        )
    }

    fn fragment_shader() -> &'static str {
        concat!(
            "#version 140\n",
            "out vec4 FragColor;",
            "void main() {",
                "FragColor = vec4(1.0);",
            "}",
        )
    }
}
