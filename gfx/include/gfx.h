#ifndef _GFX_H
#define _GFX_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdalign.h>
#include <stddef.h>
#include <stdint.h>

typedef struct gfx gfx_t;

gfx_t *gfx_new(void);
void gfx_delete(gfx_t *gfx);

void gfx_context_reset(gfx_t *gfx, const void *f(const char *));
void gfx_context_destroy(gfx_t *gfx);

void gfx_bind_framebuffer(const gfx_t *gfx, uint32_t framebuffer);
void gfx_viewport(const gfx_t *gfx, int32_t x, int32_t y, int32_t width, int32_t height);

void gfx_view(gfx_t *gfx, int32_t x, int32_t y, int32_t w, int32_t h);

void gfx_clear(const gfx_t *gfx);

void gfx_sprite(gfx_t *gfx, int32_t x, int32_t y, int32_t w, int32_t h);

void gfx_commit(gfx_t *gfx);

#ifdef __cplusplus
}
#endif

#endif
