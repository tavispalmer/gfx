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
    pub const fn as_ptr(&self) -> *const XRGB8888 {
        self.framebuffer.as_ptr() as _
    }
    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut XRGB8888 {
        self.framebuffer.as_ptr() as _
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
        for y_off in 0.max(-y) as usize..(texture.height() as isize).min(self.height() as isize - y) as usize {
            for x_off in 0.max(-x) as usize..(texture.width() as isize).min(self.width() as isize - x) as usize {
                // simple blending
                let src = &texture[y_off][x_off];
                let dst = &mut self[(y + y_off as isize) as usize][(x + x_off as isize) as usize];
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

    pub fn draw_paletted(&mut self, texture: &Texture<u8>, palette: &Texture<Color>, palette_index: usize, mut u0: usize, mut v0: usize, mut u1: usize, mut v1: usize, x: isize, y: isize) {
        let y_rev = v0 > v1;
        if y_rev {
            swap(&mut v0, &mut v1);
        }
        let x_rev = u0 > u1;
        if x_rev {
            swap(&mut u0, &mut u1);
        }
        for j in 0.max(y) as usize..0.max((y+(v1 as isize)-(v0 as isize)).min(self.height() as isize)) as usize {
            for i in 0.max(x) as usize..0.max((x+(u1 as isize)-(u0 as isize)).min(self.width() as isize)) as usize {
                // simple blending
                let v = if !y_rev {
                    (j as isize+v0 as isize-y) as usize
                } else {
                    (-(j as isize)+v1 as isize+y-1) as usize
                };
                let u = if !x_rev {
                    (i as isize+u0 as isize-x) as usize
                } else {
                    (-(i as isize)+u1 as isize+x-1) as usize
                };
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
