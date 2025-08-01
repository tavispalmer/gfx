// libretro bindings

#![allow(non_camel_case_types)]

use std::{ffi::{c_char, c_uint, c_void, CStr}, marker::PhantomData, ptr};

// 97
pub const API_VERSION: c_uint = 1;

// 443
pub const REGION_NTSC: c_uint = 0;

// 864
pub const ENVIRONMENT_SET_PIXEL_FORMAT: c_uint = 10;

// 944
pub const ENVIRONMENT_SET_HW_RENDER: c_uint = 14;

// 1050
pub const ENVIRONMENT_SET_SUPPORT_NO_GAME: c_uint = 18;

// 3973
pub type proc_address_t = Option<unsafe extern "C" fn()>;

// 5034
pub const HW_FRAME_BUFFER_VALID: *const c_void = usize::MAX as *const c_void;

// 5045
pub type hw_context_reset_t = Option<unsafe extern "C" fn()>;

// 5050
pub type hw_get_current_framebuffer_t = Option<unsafe extern "C" fn() -> usize>;

// 5053
pub type hw_get_proc_address_t = Option<unsafe extern "C" fn(sym: *const c_char) -> proc_address_t>;

// 5055
pub type hw_context_type = c_uint;

// 5064
pub const HW_CONTEXT_OPENGL_CORE: hw_context_type = 3;

// 5090
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct hw_render_callback {
    pub context_type: hw_context_type,
    pub context_reset: hw_context_reset_t,
    pub get_current_framebuffer: hw_get_current_framebuffer_t,
    pub get_proc_address: hw_get_proc_address_t,
    pub depth: bool,
    pub stencil: bool,
    pub bottom_left_origin: bool,
    pub version_major: c_uint,
    pub version_minor: c_uint,
    pub cache_context: bool,
    pub context_destroy: hw_context_reset_t,
    pub debug_context: bool,
}
impl hw_render_callback {
    pub const DEFAULT: Self = Self {
        context_type: 0,
        context_reset: None,
        get_current_framebuffer: None,
        get_proc_address: None,
        depth: false,
        stencil: false,
        bottom_left_origin: false,
        version_major: 0,
        version_minor: 0,
        cache_context: false,
        context_destroy: None,
        debug_context: false,
    };
}

// 5607
pub type pixel_format = c_uint;

// 5623
pub const PIXEL_FORMAT_XRGB8888: pixel_format = 1;

// 5971
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct system_info<'a> {
    pub library_name: *const c_char,
    pub library_version: *const c_char,
    pub valid_extensions: *const c_char,
    pub need_fullpath: bool,
    pub block_extract: bool,
    pub _marker: PhantomData<&'a ()>,
}
unsafe impl Send for system_info<'_> {}
unsafe impl Sync for system_info<'_> {}
impl Default for system_info<'_> {
    #[inline]
    fn default() -> Self {
        Self {
            library_name: ptr::null(),
            library_version: ptr::null(),
            valid_extensions: ptr::null(),
            need_fullpath: bool::default(),
            block_extract: bool::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> system_info<'a> {
    #[inline]
    pub const fn library_name(mut self, library_name: &'a CStr) -> Self {
        self.library_name = library_name.as_ptr();
        self
    }
    #[inline]
    pub const fn library_version(mut self, library_version: &'a CStr) -> Self {
        self.library_version = library_version.as_ptr();
        self
    }
    #[inline]
    pub const fn valid_extensions(mut self, valid_extensions: &'a CStr) -> Self {
        self.valid_extensions = valid_extensions.as_ptr();
        self
    }
    #[inline]
    pub const fn need_fullpath(mut self, need_fullpath: bool) -> Self {
        self.need_fullpath = need_fullpath;
        self
    }
    #[inline]
    pub const fn block_extract(mut self, block_extract: bool) -> Self {
        self.block_extract = block_extract;
        self
    }
}

// 6234
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct game_geometry {
    pub base_width: c_uint,
    pub base_height: c_uint,
    pub max_width: c_uint,
    pub max_height: c_uint,
    pub aspect_ratio: f32,
}
impl Default for game_geometry {
    #[inline]
    fn default() -> Self {
        Self {
            base_width: c_uint::default(),
            base_height: c_uint::default(),
            max_width: c_uint::default(),
            max_height: c_uint::default(),
            aspect_ratio: f32::default(),
        }
    }
}
impl game_geometry {
    #[inline]
    pub const fn base_width(mut self, base_width: c_uint) -> Self {
        self.base_width = base_width;
        self
    }
    #[inline]
    pub const fn base_height(mut self, base_height: c_uint) -> Self {
        self.base_height = base_height;
        self
    }
    #[inline]
    pub const fn max_width(mut self, max_width: c_uint) -> Self {
        self.max_width = max_width;
        self
    }
    #[inline]
    pub const fn max_height(mut self, max_height: c_uint) -> Self {
        self.max_height = max_height;
        self
    }
    #[inline]
    pub const fn aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }
}

// 6286
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct system_timing {
    pub fps: f64,
    pub sample_rate: f64,
}
impl Default for system_timing {
    #[inline]
    fn default() -> Self {
        Self {
            fps: f64::default(),
            sample_rate: f64::default(),
        }
    }
}
impl system_timing {
    #[inline]
    pub const fn fps(mut self, fps: f64) -> Self {
        self.fps = fps;
        self
    }
    #[inline]
    pub const fn sample_rate(mut self, sample_rate: f64) -> Self {
        self.sample_rate = sample_rate;
        self
    }
}

// 6300
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct system_av_info {
    pub geometry: game_geometry,
    pub timing: system_timing,
}
impl Default for system_av_info {
    #[inline]
    fn default() -> Self {
        Self {
            geometry: game_geometry::default(),
            timing: system_timing::default(),
        }
    }
}
impl system_av_info {
    #[inline]
    pub const fn geometry(mut self, geometry: game_geometry) -> Self {
        self.geometry = geometry;
        self
    }
    #[inline]
    pub const fn timing(mut self, timing: system_timing) -> Self {
        self.timing = timing;
        self
    }
}

// 6820
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct game_info<'a> {
    pub path: *const c_char,
    pub data: *const c_void,
    pub size: usize,
    pub meta: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for game_info<'_> {
    #[inline]
    fn default() -> Self {
        Self {
            path: ptr::null(),
            data: ptr::null(),
            size: usize::default(),
            meta: ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> game_info<'a> {
    #[inline]
    pub const fn path(mut self, path: &'a CStr) -> Self {
        self.path = path.as_ptr();
        self
    }
    #[inline]
    pub const fn data<T>(mut self, data: &'a [T]) -> Self {
        self.data = data.as_ptr() as *const c_void;
        self.size = data.len() * size_of::<T>();
        self
    }
    #[inline]
    pub const fn meta(mut self, meta: &'a CStr) -> Self {
        self.meta = meta.as_ptr();
        self
    }
}

pub type environment_t = unsafe extern "C" fn(cmd: c_uint, data: *mut c_void) -> bool;

pub type video_refresh_t = unsafe extern "C" fn(data: *const c_void, width: c_uint, height: c_uint, pitch: usize);

pub type audio_sample_t = unsafe extern "C" fn(left: i16, right: i16);

pub type audio_sample_batch_t = unsafe extern "C" fn(data: *const i16, frames: usize) -> usize;

pub type input_poll_t = unsafe extern "C" fn();

pub type input_state_t = unsafe extern "C" fn(port: c_uint, device: c_uint, index: c_uint, id: c_uint) -> i16;

