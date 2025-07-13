use glm::mat4;

pub struct SpriteOptions {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub tex_x: usize,
    pub tex_y: usize,
}

impl SpriteOptions {
    pub const DEFAULT: SpriteOptions = SpriteOptions {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        tex_x: 0,
        tex_y: 0,
    };
    #[inline]
    pub const fn x(mut self, x: usize) -> Self {
        self.x = x;
        self
    }
    #[inline]
    pub const fn y(mut self, y: usize) -> Self {
        self.y = y;
        self
    }
    #[inline]
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    #[inline]
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    #[inline]
    pub const fn tex_x(mut self, tex_x: usize) -> Self {
        self.tex_x = tex_x;
        self
    }
    #[inline]
    pub const fn tex_y(mut self, tex_y: usize) -> Self {
        self.tex_y = tex_y;
        self
    }
}

impl Default for SpriteOptions {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}