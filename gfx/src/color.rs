#[derive(Clone, Copy, Debug)]
pub struct Color(u32);

impl Color {
    pub const WHITE: Self = Self::rgb(0xFF, 0xFF, 0xFF);
    pub const SILVER: Self = Self::rgb(0xC0, 0xC0, 0xC0);
    pub const GRAY: Self = Self::rgb(0x80, 0x80, 0x80);
    pub const BLACK: Self = Self::rgb(0x00, 0x00, 0x00);
    pub const RED: Self = Self::rgb(0xFF, 0x00, 0x00);
    pub const MAROON: Self = Self::rgb(0x80, 0x00, 0x00);
    pub const YELLOW: Self = Self::rgb(0xFF, 0xFF, 0x00);
    pub const OLIVE: Self = Self::rgb(0x80, 0x80, 0x00);
    pub const LIME: Self = Self::rgb(0x00, 0xFF, 0x00);
    pub const GREEN: Self = Self::rgb(0x00, 0x80, 0x00);
    pub const AQUA: Self = Self::rgb(0x00, 0xFF, 0xFF);
    pub const TEAL: Self = Self::rgb(0x00, 0x80, 0x80);
    pub const BLUE: Self = Self::rgb(0x00, 0x00, 0xFF);
    pub const NAVY: Self = Self::rgb(0x00, 0x00, 0x80);
    pub const FUCHSIA: Self = Self::rgb(0xFF, 0x00, 0xFF);
    pub const PURPLE: Self = Self::rgb(0x80, 0x00, 0x80);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 0xFF)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub const fn r(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub const fn b(&self) -> u8 {
        self.0 as u8
    }

    pub const fn a(&self) -> u8 {
        (self.0 >> 24) as u8
    }
}
