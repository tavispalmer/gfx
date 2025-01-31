use std::{alloc::{alloc, dealloc, Layout}, ops::{Index, IndexMut}, ptr::NonNull, slice};

use crate::Color;

pub struct Texture<C> {
    texture: NonNull<C>,
    width: usize,
    height: usize,
}

impl<C: Copy> Texture<C> {
    #[inline]
    pub fn new(texture: &[C], width: usize, height: usize) -> Self {
        Self {
            texture: unsafe { 
                let tex = NonNull::new(alloc(Layout::array::<C>(width * height).unwrap()) as *mut C).unwrap();
                slice::from_raw_parts_mut(tex.as_ptr(), width * height).copy_from_slice(texture);
                tex
            },
            width,
            height,
        }
    }
}

impl Texture<u8> {
    pub fn from_2bpp(data: &[u8], width: usize, height: usize) -> Self {
        let mut graphics_data = Vec::with_capacity((width*height)>>2);
        // for each row
        for i in 0..(height>>3) {
            // for each row of each tile
            for k in 0..0x8 {
                // for each tile
                for j in 0..(width>>3) {
                    let chr1 = data[((i*width)<<1)+(j<<4)+k];
                    let chr2 = data[((i*width)<<1)+(j<<4)+k+0x8];
                    graphics_data.push(((chr1 >> 7) & 1) | ((chr2 >> 6) & 2));
                    graphics_data.push(((chr1 >> 6) & 1) | ((chr2 >> 5) & 2));
                    graphics_data.push(((chr1 >> 5) & 1) | ((chr2 >> 4) & 2));
                    graphics_data.push(((chr1 >> 4) & 1) | ((chr2 >> 3) & 2));
                    graphics_data.push(((chr1 >> 3) & 1) | ((chr2 >> 2) & 2));
                    graphics_data.push(((chr1 >> 2) & 1) | ((chr2 >> 1) & 2));
                    graphics_data.push(((chr1 >> 1) & 1) | (chr2 & 2));
                    graphics_data.push((chr1 & 1) | ((chr2 << 1) & 2));
                }
            }
        }
        Self::new(&graphics_data[..], width, height)
    }
}

impl<C> Texture<C> {
    #[inline]
    pub const fn width(&self) -> usize {
        self.width
    }
    #[inline]
    pub const fn height(&self) -> usize {
        self.height
    }
}

impl<C> Index<usize> for Texture<C> {
    type Output = [C];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            &slice::from_raw_parts(
                self.texture.as_ptr(),
                self.width * self.height
            )[(index*self.width)..((index+1)*self.width)]
        }
    }
}

impl<C> IndexMut<usize> for Texture<C> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            &mut slice::from_raw_parts_mut(
                self.texture.as_ptr(),
                self.width * self.height
            )[(index*self.width)..((index+1)*self.width)]
        }
    }
}

impl<C> Drop for Texture<C> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            dealloc(self.texture.as_ptr() as *mut u8, Layout::array::<Color>(self.width * self.height).unwrap());
        }
    }
}
