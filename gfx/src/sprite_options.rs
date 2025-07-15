use std::rc::Rc;

use crate::TextureGL;

pub struct SpriteOptions {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub tex_x: usize,
    pub tex_y: usize,

    pub tex: Option<Rc<TextureGL>>,
}

impl SpriteOptions {
    pub const DEFAULT: SpriteOptions = SpriteOptions {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        tex_x: 0,
        tex_y: 0,
        tex: None,
    };
    #[inline]
    pub const fn x(&mut self, x: usize) -> &mut Self {
        self.x = x;
        self
    }
    #[inline]
    pub const fn y(&mut self, y: usize) -> &mut Self {
        self.y = y;
        self
    }
    #[inline]
    pub const fn width(&mut self, width: usize) -> &mut Self {
        self.width = width;
        self
    }
    #[inline]
    pub const fn height(&mut self, height: usize) -> &mut Self {
        self.height = height;
        self
    }
    #[inline]
    pub const fn tex_x(&mut self, tex_x: usize) -> &mut Self {
        self.tex_x = tex_x;
        self
    }
    #[inline]
    pub const fn tex_y(&mut self, tex_y: usize) -> &mut Self {
        self.tex_y = tex_y;
        self
    }
    #[inline]
    pub fn tex(&mut self, tex: Option<Rc<TextureGL>>) -> &mut Self {
        self.tex = tex;
        self
    }
}

impl Default for SpriteOptions {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}