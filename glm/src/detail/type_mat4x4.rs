use std::{ops::{Deref, DerefMut, Index, IndexMut}, slice::SliceIndex};

use crate::vec4;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct mat4x4<T = f32> {
    value: [vec4<T>; 4],
}

impl mat4x4<f32> {
    pub const DEFAULT: Self = Self::new(
        vec4::new(1.0, 0.0, 0.0, 0.0),
        vec4::new(0.0, 1.0, 0.0, 0.0),
        vec4::new(0.0, 0.0, 1.0, 0.0),
        vec4::new(0.0, 0.0, 0.0, 1.0),
    );
}

impl Default for mat4x4<f32> {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl<T> mat4x4<T> {
    #[inline]
    pub const fn new(
        v0: vec4<T>,
        v1: vec4<T>,
        v2: vec4<T>,
        v3: vec4<T>,
    ) -> Self {
        Self {
            value: [v0, v1, v2, v3],
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const vec4<T> {
        self.value.as_ptr()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut vec4<T> {
        self.value.as_mut_ptr()
    }

    #[inline]
    pub const fn as_slice(&self) -> &[vec4<T>] {
        &self.value
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [vec4<T>] {
        &mut self.value
    }
}

impl<T> Deref for mat4x4<T> {
    type Target = [vec4<T>];

    #[inline]
    fn deref(&self) -> &[vec4<T>] {
        self.as_slice()
    }
}

impl<T> DerefMut for mat4x4<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [vec4<T>] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[vec4<T>]>> Index<I> for mat4x4<T> {
    type Output = I::Output;
    
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[vec4<T>]>> IndexMut<I> for mat4x4<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}
