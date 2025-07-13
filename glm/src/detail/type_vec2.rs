use std::{ffi::{c_int, c_uint}, ops::{Add, Deref, DerefMut, Index, IndexMut, Mul}, slice::{self, SliceIndex}};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct vec2<T = f32> {
    pub x: T,
    pub y: T,
}

#[allow(non_camel_case_types)]
pub type bvec2 = vec2<bool>;
#[allow(non_camel_case_types)]
pub type dvec2 = vec2<f64>;
#[allow(non_camel_case_types)]
pub type ivec2 = vec2<c_int>;
#[allow(non_camel_case_types)]
pub type uvec2 = vec2<c_uint>;

impl<T> vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
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

impl<T> Deref for vec2<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> DerefMut for vec2<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for vec2<T> {
    type Output = I::Output;
    
    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for vec2<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(&mut **self, index)
    }
}

impl<T: Add<U>, U> Add<vec2<U>> for vec2<T> {
    type Output = vec2<<T as Add<U>>::Output>;

    #[inline]
    fn add(self, rhs: vec2<U>) -> Self::Output {
        vec2::new(
            self.x + rhs.x,
            self.y + rhs.y,
        )
    }
}

impl<T: Mul<U>, U: Copy> Mul<U> for vec2<T> {
    type Output = vec2<<T as Mul<U>>::Output>;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        vec2::new(
            self.x * rhs,
            self.y * rhs,
        )
    }
}
