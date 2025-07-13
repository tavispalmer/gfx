#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub vertex_tl: glm::vec2,
    pub vertex_tr: glm::vec2,
    pub vertex_bl: glm::vec2,
    pub vertex_br: glm::vec2,
}

impl Quad {
    pub const fn new(
        vertex_tl: glm::vec2,
        vertex_tr: glm::vec2,
        vertex_bl: glm::vec2,
        vertex_br: glm::vec2,
    ) -> Self {
        Self {
            vertex_tl,
            vertex_tr,
            vertex_bl,
            vertex_br,
        }
    }
}
