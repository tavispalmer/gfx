use std::{ffi::{c_int, c_uint}, ops::{Deref, DerefMut, Index, IndexMut}, slice::{self, SliceIndex}};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct vec4<T = f32> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[allow(non_camel_case_types)]
pub type bvec4 = vec4<bool>;
#[allow(non_camel_case_types)]
pub type dvec4 = vec4<f64>;
#[allow(non_camel_case_types)]
pub type ivec4 = vec4<c_int>;
#[allow(non_camel_case_types)]
pub type uvec4 = vec4<c_uint>;

impl<T> vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        size_of::<Self>() / size_of::<T>()
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        &raw const *self as *const T
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        &raw mut *self as *mut T
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T> Deref for vec4<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for vec4<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for vec4<T> {
    type Output = I::Output;
    
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for vec4<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}
