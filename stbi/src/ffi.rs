use std::ffi::{c_char, c_int, c_uint, c_void};

pub const STBI_default: c_uint = 0;
pub const STBI_grey: c_uint = 1;
pub const STBI_grey_alpha: c_uint = 2;
pub const STBI_rgb: c_uint = 3;
pub const STBI_rgb_alpha: c_uint = 4;

pub type stbi_uc = u8;
pub type stbi_us = u16;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct stbi_io_callbacks {
    pub read: Option<unsafe extern "C" fn(user: *mut c_void, data: *mut c_char, size: c_int) -> c_int>,
    pub skip: Option<unsafe extern "C" fn(user: *mut c_void, n: c_int)>,
    pub eof: Option<unsafe extern "C" fn(user: *mut c_void) -> c_int>,
}

unsafe extern "C" {
    pub unsafe fn stbi_load_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut c_void, x: *mut c_int, y: *mut c_int, channels_in_file: *mut c_int, desired_channels: c_int) -> *mut stbi_uc;
    pub unsafe fn stbi_image_free(retval_from_stbi_load: *mut c_void);
}
