use std::{ffi::{c_char, c_uint, c_void}, mem::{self, MaybeUninit}, ptr};

use app::App;

mod app;
mod retro;

// app
static mut APP: MaybeUninit<App> = MaybeUninit::uninit();

// libretro callbacks
static mut ENVIRON_CB: retro::environment_t = {
    unsafe extern "C" fn environ_cb(_cmd: c_uint, _data: *mut c_void) -> bool {
        panic!("Unable to load environ_cb")
    }
    environ_cb
};
static mut VIDEO_CB: retro::video_refresh_t = {
    unsafe extern "C" fn video_cb(_data: *const c_void, _width: c_uint, _height: c_uint, _pitch: usize) {
        panic!("Unable to load video_cb")
    }
    video_cb
};
static mut AUDIO_CB: retro::audio_sample_t = {
    unsafe extern "C" fn audio_cb(_left: i16, _right: i16) {
        panic!("Unable to load audio_cb")
    }
    audio_cb
};
static mut AUDIO_BATCH_CB: retro::audio_sample_batch_t = {
    unsafe extern "C" fn audio_batch_cb(_data: *const i16, _frames: usize) -> usize {
        panic!("Unable to load audio_batch_cb")
    }
    audio_batch_cb
};
static mut INPUT_POLL_CB: retro::input_poll_t = {
    unsafe extern "C" fn input_poll_cb() {
        panic!("Unable to load input_poll_cb")
    }
    input_poll_cb
};
static mut INPUT_STATE_CB: retro::input_state_t = {
    unsafe extern "C" fn input_state_cb(_port: c_uint, _device: c_uint, _index: c_uint, _id: c_uint) -> i16 {
        panic!("Unable to load input_state_cb")
    }
    input_state_cb
};

// 3d rendering callbacks
static mut HW_RENDER: retro::hw_render_callback = retro::hw_render_callback::DEFAULT;

extern "C" fn context_reset() {
    eprintln!("Context reset!");
    unsafe {
        #[allow(static_mut_refs)]
        APP.assume_init_mut().context_reset(|sym|
            mem::transmute(HW_RENDER.get_proc_address.unwrap_unchecked()(sym.as_ptr()))
        );
    }
}

extern "C" fn context_destroy() {
    unsafe {
        #[allow(static_mut_refs)]
        APP.assume_init_mut().context_destroy();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_environment(cb: retro::environment_t) {
    unsafe {
        ENVIRON_CB = cb;

        // set support for no game
        let no_rom = true;
        ENVIRON_CB(
            retro::ENVIRONMENT_SET_SUPPORT_NO_GAME,
            &raw const no_rom as _,
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_video_refresh(cb: retro::video_refresh_t) {
    unsafe {
        VIDEO_CB = cb;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample(cb: retro::audio_sample_t) {
    unsafe {
        AUDIO_CB = cb;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_audio_sample_batch(cb: retro::audio_sample_batch_t) {
    unsafe {
        AUDIO_BATCH_CB = cb;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_poll(cb: retro::input_poll_t) {
    unsafe {
        INPUT_POLL_CB = cb;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_input_state(cb: retro::input_state_t) {
    unsafe {
        INPUT_STATE_CB = cb;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_init() {}

#[unsafe(no_mangle)]
pub extern "C" fn retro_deinit() {}

#[unsafe(no_mangle)]
pub extern "C" fn retro_api_version() -> c_uint {
    retro::API_VERSION
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_info(info: *mut retro::system_info) {
    unsafe {
        *info = retro::system_info::default()
            .library_name(c"Example")
            .library_version(c"v1");
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_system_av_info(info: *mut retro::system_av_info) {
    unsafe {
        *info = retro::system_av_info::default()
            .geometry(retro::game_geometry::default()
                .base_width(App::WIDTH as u32)
                .base_height(App::HEIGHT as u32)
                .max_width(App::WIDTH as u32)
                .max_height(App::HEIGHT as u32)
                .aspect_ratio(App::ASPECT_RATIO))
            .timing(retro::system_timing::default()
                .fps(60.0));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_set_controller_port_device(port: c_uint, device: c_uint) {}

#[unsafe(no_mangle)]
pub extern "C" fn retro_reset() {}

#[unsafe(no_mangle)]
pub extern "C" fn retro_run() {
    unsafe {
        #[allow(static_mut_refs)] {
            APP.assume_init_mut().set_framebuffer(
                HW_RENDER.get_current_framebuffer.unwrap_unchecked()() as u32,
            );
            APP.assume_init_mut().run();
        }
        VIDEO_CB(
            retro::HW_FRAME_BUFFER_VALID,
            App::WIDTH as u32,
            App::HEIGHT as u32,
            0,
        )
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize_size() -> usize {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_serialize(data: *mut c_void, len: usize) -> bool {
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unserialize(data: *const c_void, len: usize) -> bool {
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_reset() {

}

#[unsafe(no_mangle)]
pub extern "C" fn retro_cheat_set(index: c_uint, enabled: bool, code: *const c_char) {

}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game(game: *const retro::game_info) -> bool {
    unsafe {
        let fmt = retro::PIXEL_FORMAT_XRGB8888;
        if !ENVIRON_CB(
            retro::ENVIRONMENT_SET_PIXEL_FORMAT,
            &raw const fmt as _,
        ) {
            eprintln!("XRGB8888 is not supported.");
            return false;
        }

        // init hw context
        HW_RENDER.context_type = retro::HW_CONTEXT_OPENGL_CORE;
        HW_RENDER.version_major = 3;
        HW_RENDER.version_minor = 1;
        HW_RENDER.context_reset = Some(context_reset);
        HW_RENDER.context_destroy = Some(context_destroy);
        HW_RENDER.depth = true;
        HW_RENDER.stencil = true;
        HW_RENDER.bottom_left_origin = true;

        if !ENVIRON_CB(
            retro::ENVIRONMENT_SET_HW_RENDER,
            &raw mut HW_RENDER as _,
        ) {
            eprintln!("HW Context could not be initialized, exiting...");
            return false;
        }

        // initialize app
        APP = MaybeUninit::new(App::new());
    }

    eprintln!("Loaded game!");
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_load_game_special(game_type: c_uint, info: *const retro::game_info, num_info: usize) -> bool {
    false
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_unload_game() {
    unsafe {
        #[allow(static_mut_refs)]
        APP.assume_init_drop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_region() -> c_uint {
    retro::REGION_NTSC
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_data(id: c_uint) -> *mut c_void {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn retro_get_memory_size(id: c_uint) -> usize {
    0
}
