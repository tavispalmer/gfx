use crate::gfx::{Vertex, VertexAttrib, VertexFormat, VertexUsage};

#[repr(C)]
pub struct PosTex {
    pub pos: glm::vec2,
    pub tex: glm::vec2,
}

impl PosTex {
    #[inline]
    pub const fn new(pos: glm::vec2, tex: glm::vec2) -> Self {
        Self {
            pos,
            tex,
        }
    }
}

impl Vertex for PosTex {
    fn vertex_attrib() -> &'static [VertexAttrib] {
        const VERTEX_ATTRIB: [VertexAttrib; 2] = [
            VertexAttrib::new(
                VertexUsage::Position,
                VertexFormat::Vec2,
                0,
            ),
            VertexAttrib::new(
                VertexUsage::TextureCoordinate,
                VertexFormat::Vec2,
                size_of::<glm::vec2>(),
            ),
        ];
        &VERTEX_ATTRIB
    }

    fn vertex_shader() -> &'static str {
        concat!(
            "#version 140\n",
            "uniform mat4 mat;",
            "in vec2 pos;",
            "in vec2 tex;",
            "out vec2 frag_tex;",
            "void main() {",
                "gl_Position = mat * vec4(pos, 0.0, 1.0);",
                "frag_tex = tex;",
            "}",
        )
    }

    fn fragment_shader() -> &'static str {
        concat!(
            "#version 140\n",
            "uniform sampler2D tex2d;",
            "in vec2 frag_tex;",
            "out vec4 FragColor;",
            "void main() {",
                "FragColor = texture(tex2d, frag_tex);",
            "}",
        )
    }
}
