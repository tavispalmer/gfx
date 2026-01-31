use std::{ffi::c_void, mem::MaybeUninit, rc::Rc, slice};

use crate::gl::{self, Gl};

pub struct Texture {
    gl: Rc<Gl>,
    texture: u32,
    tile_w: usize,
    tile_h: usize,
    tiles_w: usize,
    tiles_h: usize,
    size: usize,
}

impl Texture {
    #[inline]
    pub fn tile_w(&self) -> usize {
        self.tile_w
    }

    #[inline]
    pub fn tile_h(&self) -> usize {
        self.tile_h
    }

    #[inline]
    pub fn tiles_w(&self) -> usize {
        self.tiles_w
    }

    #[inline]
    pub fn tiles_h(&self) -> usize {
        self.tiles_h
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn load_1bpp(gl: Rc<Gl>, buf: &[u8], w: usize, h: usize) -> Self {
        // how many tiles?
        let tile_len = (w + 7) / 8 * h;
        let num_tiles = buf.len() / tile_len;

        // try to fit it in a power of 2 square
        let mut size = 1;
        let mut tiles_w;
        let mut tiles_h;
        loop {
            tiles_w = size / w;
            tiles_h = size / h;
            let max_tiles = tiles_w * tiles_h;

            if max_tiles >= num_tiles {
                break;
            }

            size *= 2;
        }

        // we have our size: now generate an image.
        let mut pixels = vec![0; size * size];
        for tile in 0..num_tiles {
            for y in 0..h {
                for x in 0..8 {
                    pixels[((tile / tiles_w) * h + y) * size + (tile % tiles_w) * w + x] =
                        (buf[tile * h + y] << x) & 0x80;
                }
            }
        }

        let texture;
        unsafe {
            let gl = gl.as_ref();

            let mut textures = MaybeUninit::uninit();
            gl.gen_textures(slice::from_raw_parts_mut(textures.as_mut_ptr(), 1));
            texture = textures.assume_init();

            gl.bind_texture(gl::TEXTURE_2D, texture);

            gl.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            gl.tex_image_2d(
                gl::TEXTURE_2D,
                0,
                gl::RED as i32,
                size as i32,
                size as i32,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const c_void,
            );
        }

        Self {
            gl,
            texture,
            tile_w: w,
            tile_h: h,
            tiles_w: size / w,
            tiles_h: size / h,
            size,
        }
    }
}
