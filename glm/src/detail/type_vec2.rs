use std::ffi::{c_int, c_uint};

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
    pub const fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
