#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quad<T> {
    pub vertex_tl: T,
    pub vertex_tr: T,
    pub vertex_bl: T,
    pub vertex_br: T,
}

impl<T> Quad<T> {
    pub const fn new(
        vertex_tl: T,
        vertex_tr: T,
        vertex_bl: T,
        vertex_br: T,
    ) -> Self {
        Self {
            vertex_tl,
            vertex_tr,
            vertex_bl,
            vertex_br,
        }
    }
}
