#include <gfx.h>
#include <libretro.h>
#include <stdio.h>
#include <string.h>

static gfx_t *gfx;

static retro_environment_t environ_cb;
static retro_video_refresh_t video_cb;
static retro_audio_sample_t audio_cb;
static retro_audio_sample_batch_t audio_batch_cb;
static retro_input_poll_t input_poll_cb;
static retro_input_state_t input_state_cb;

static struct retro_hw_render_callback hw_render;

static void context_reset(void) {
    gfx_context_reset(gfx, (const void *(*)(const char *))hw_render.get_proc_address);
}

static void context_destroy(void) {
    gfx_context_destroy(gfx);
}

void retro_set_environment(retro_environment_t cb) {
    environ_cb = cb;

    bool no_rom = true;
    environ_cb(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, &no_rom);
}

void retro_set_video_refresh(retro_video_refresh_t cb) {
    video_cb = cb;
}

void retro_set_audio_sample(retro_audio_sample_t cb) {
    audio_cb = cb;
}

void retro_set_audio_sample_batch(retro_audio_sample_batch_t cb) {
    audio_batch_cb = cb;
}

void retro_set_input_poll(retro_input_poll_t cb) {
    input_poll_cb = cb;
}

void retro_set_input_state(retro_input_state_t cb) {
    input_state_cb = cb;
}

void retro_init(void) {

}

void retro_deinit(void) {

}

unsigned retro_api_version() {
    return RETRO_API_VERSION;
}

void retro_get_system_info(struct retro_system_info *info) {
    *info = (struct retro_system_info) {
        .library_name = "Gfx Example",
        .library_version = "v1",
    };
}

void retro_get_system_av_info(struct retro_system_av_info *info) {
    *info = (struct retro_system_av_info) {
        .geometry = {
            .base_width = 800,
            .base_height = 600,
            .max_width = 800,
            .max_height = 600,
        },
        .timing = {
            .fps = 60.0,
        }
    };
}

void retro_set_controller_port_device(unsigned port, unsigned device) {

}

void retro_reset(void) {

}

void retro_run(void) {
    gfx_bind_framebuffer(gfx, hw_render.get_current_framebuffer());
    gfx_viewport(gfx, 0, 0, 800, 600);
    gfx_clear(gfx);
    gfx_sprite(gfx, 0, 0);
    gfx_commit(gfx);
    video_cb(RETRO_HW_FRAME_BUFFER_VALID, 800, 600, 0);
}

size_t retro_serialize_size(void) {
    return 0;
}

bool retro_serialize(void *data, size_t len) {
    return false;
}

bool retro_unserialize(const void *data, size_t len) {
    return false;
}

void retro_cheat_reset(void) {

}

void retro_cheat_set(unsigned index, bool enabled, const char *code) {

}

bool retro_load_game(const struct retro_game_info *game) {
    enum retro_pixel_format fmt = RETRO_PIXEL_FORMAT_XRGB8888;
    if (!environ_cb(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, &fmt)) {
        fprintf(stderr, "XRGB8888 is not supported.\n");
        return false;
    }

    gfx = gfx_new();

    hw_render.context_type = RETRO_HW_CONTEXT_OPENGL_CORE;
    hw_render.version_major = 3;
    hw_render.version_minor = 1;
    hw_render.context_reset = context_reset;
    hw_render.context_destroy = context_destroy;
    hw_render.depth = true;
    hw_render.stencil = true;
    hw_render.bottom_left_origin = true;

    if (!environ_cb(RETRO_ENVIRONMENT_SET_HW_RENDER, &hw_render)) {
        fprintf(stderr, "HW Context could not be initialized, exiting...\n");
        return false;
    }

    fprintf(stderr, "Loaded game!\n");
    return true;
}

bool retro_load_game_special(unsigned game_type, const struct retro_game_info *info, size_t num_info) {
    return false;
}

void retro_unload_game(void) {
    gfx_delete(gfx);
}

unsigned retro_get_region(void) {
    return RETRO_REGION_NTSC;
}

void *retro_get_memory_data(unsigned id) {
    return NULL;
}

size_t retro_get_memory_size(unsigned id) {
    return 0;
}
