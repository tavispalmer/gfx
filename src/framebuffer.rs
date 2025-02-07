use core::slice;
use std::{alloc::{alloc_zeroed, dealloc, Layout}, mem::swap, ops::{Index, IndexMut}, ptr::NonNull};

use crate::{Color, color::XRGB8888, texture::Texture};

pub struct Framebuffer {
    framebuffer: NonNull<XRGB8888>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            framebuffer: unsafe {
                // use NonNull instead of Box so we can set the alignment
                NonNull::new(alloc_zeroed(Layout::from_size_align(
                    width.next_power_of_two() * height * size_of::<XRGB8888>(),
                    width.next_power_of_two() * size_of::<XRGB8888>()
                ).unwrap()) as *mut XRGB8888).unwrap()
            },
            width,
            height,
        }
    }
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }
    #[inline]
    pub const fn pitch(&self) -> usize {
        self.width.next_power_of_two() * size_of::<XRGB8888>()
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.framebuffer.as_ptr() as _
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.framebuffer.as_ptr() as _
    }
    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.height()*self.pitch()) }
    }
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.height()*self.pitch()) }
    }

    pub fn clear(&mut self, color: Color) {
        let xrgb8888 = XRGB8888::new(color.r(), color.g(), color.b());
        unsafe {
            slice::from_raw_parts_mut(
                self.framebuffer.as_ptr(),
                self.width.next_power_of_two() * self.height).fill(xrgb8888);
        }
    }

    pub fn draw(&mut self, texture: &Texture<Color>, x: isize, y: isize) {
        for j in 0.max(y) as usize..0.max((y+(texture.height() as isize)).min(self.height() as isize)) as usize {
            // texture coordinates
            let v = (j as isize-y) as usize;

            for i in 0.max(x) as usize..0.max((x+(texture.width() as isize)).min(self.width() as isize)) as usize {
                let u = (i as isize-x) as usize;

                // simple blending
                let src = &texture[v][u];
                let dst = &mut self[j][i];
                let src_mul = src.a() as u16;
                let dst_mul = 0xff - src_mul;
            
                *dst = XRGB8888::new(
                    ((dst.r() as u16 * dst_mul + src.r() as u16 * src_mul) / 0xff) as u8,
                    ((dst.g() as u16 * dst_mul + src.g() as u16 * src_mul) / 0xff) as u8,
                    ((dst.b() as u16 * dst_mul + src.b() as u16 * src_mul) / 0xff) as u8,
                );
            }
        }
    }

    pub fn draw_paletted(&mut self, texture: &Texture<u8>, x: isize, y: isize, u: usize, v: usize, width: usize, height: usize, palette: &Texture<Color>, palette_index: usize, flip_x: bool, flip_y: bool) {
        for j in 0.max(y) as usize..0.max((y+(height as isize)).min(self.height() as isize)) as usize {
            // texture coordinates
            let v = if !flip_y {
                ((j+v) as isize-y) as usize
            } else {
                ((v+height-j) as isize+y-1) as usize
            };

            for i in 0.max(x) as usize..0.max((x+(width as isize)).min(self.width() as isize)) as usize {
                let u = if !flip_x {
                    ((i+u) as isize-x) as usize
                } else {
                    ((u+width-i) as isize+x-1) as usize
                };

                // simple blending
                let src = &palette[palette_index][texture[v][u] as usize];
                let dst = &mut self[j][i];
                let src_mul = src.a() as u16;
                let dst_mul = 0xff - src_mul;
            
                *dst = XRGB8888::new(
                    ((dst.r() as u16 * dst_mul + src.r() as u16 * src_mul) / 0xff) as u8,
                    ((dst.g() as u16 * dst_mul + src.g() as u16 * src_mul) / 0xff) as u8,
                    ((dst.b() as u16 * dst_mul + src.b() as u16 * src_mul) / 0xff) as u8,
                );
            }
        }
    }
}

impl Drop for Framebuffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            dealloc(self.framebuffer.as_ptr() as *mut u8, Layout::from_size_align(
                self.width.next_power_of_two() * self.height * size_of::<XRGB8888>(),
                self.width.next_power_of_two() * size_of::<XRGB8888>()
            ).unwrap());
        }
    }
}

// 2D indexing
impl Index<usize> for Framebuffer {
    type Output = [XRGB8888];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let pitch = self.width.next_power_of_two();
        unsafe {
            &slice::from_raw_parts(
                self.framebuffer.as_ptr(),
                self.width.next_power_of_two() * self.height
            )[(index*pitch)..((index+1)*pitch)]
        }
    }
}

impl IndexMut<usize> for Framebuffer {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let pitch = self.width.next_power_of_two();
        unsafe {
            &mut slice::from_raw_parts_mut(
                self.framebuffer.as_ptr(),
                self.width.next_power_of_two() * self.height
            )[(index*pitch)..((index+1)*pitch)]
        }
    }
}
