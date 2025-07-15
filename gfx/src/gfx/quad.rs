#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quad {
    pub pos_tl: glm::vec2,
    pub tex_tl: glm::vec2,
    pub pos_tr: glm::vec2,
    pub tex_tr: glm::vec2,
    pub pos_bl: glm::vec2,
    pub tex_bl: glm::vec2,
    pub pos_br: glm::vec2,
    pub tex_br: glm::vec2,
}

impl Quad {
    pub const fn new(
        pos_tl: glm::vec2,
        tex_tl: glm::vec2,
        pos_tr: glm::vec2,
        tex_tr: glm::vec2,
        pos_bl: glm::vec2,
        tex_bl: glm::vec2,
        pos_br: glm::vec2,
        tex_br: glm::vec2,
    ) -> Self {
        Self {
            pos_tl,
            tex_tl,
            pos_tr,
            tex_tr,
            pos_bl,
            tex_bl,
            pos_br,
            tex_br,
        }
    }
}
