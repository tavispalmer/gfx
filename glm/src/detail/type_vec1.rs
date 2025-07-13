use std::ffi::{c_int, c_uint};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub struct vec1<T = f32> {
    pub x: T,
}

#[allow(non_camel_case_types)]
pub type bvec1 = vec1<bool>;
#[allow(non_camel_case_types)]
pub type dvec1 = vec1<f64>;
#[allow(non_camel_case_types)]
pub type ivec1 = vec1<c_int>;
#[allow(non_camel_case_types)]
pub type uvec1 = vec1<c_uint>;

impl<T> vec1<T> {
    pub const fn new(x: T) -> Self {
        Self {
            x,
        }
    }
}
