use std::{ffi::c_void, io::Result, mem::MaybeUninit, path::Path, rc::Rc};

use crate::{gl, GL};

struct TextureInner {
    // vtable
    gl: Rc<GL>,
    tex: u32,
}

impl Drop for TextureInner {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_textures(&[self.tex]);
        }
    }
}

pub struct TextureGL(Rc<TextureInner>);

impl TextureGL {
    pub fn open<P: AsRef<Path>>(gl: Rc<GL>, path: P) -> Result<Self> {
        Self::open_erased(gl, path.as_ref())
    }

    fn open_erased(gl: Rc<GL>, path: &Path) -> Result<Self> {
        // here we use stbi
        let image = stbi::load(path, None)?;

        // generate the opengl texture
        unsafe {
            let mut textures = [0];
            gl.gen_textures(&mut textures);
            let tex = textures[0];

            gl.bind_texture(gl::TEXTURE_2D, tex);

            gl.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

            let format = match image.channels_in_file() {
                stbi::Channels::Grey => gl::RED,
                stbi::Channels::GreyAlpha => gl::RG,
                stbi::Channels::Rgb => gl::RGB,
                stbi::Channels::RgbAlpha => gl::RGBA,
            };
            gl.tex_image_2d(
                gl::TEXTURE_2D,
                0,
                format as i32,
                image.x() as i32,
                image.y() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                image.as_ptr() as *const c_void,
            );

            Ok(Self(Rc::new(TextureInner {
                gl,
                tex,
            })))
        }
    }

    pub(crate) fn id(&self) -> u32 {
        self.0.tex
    }

    pub(crate) fn bind(&self) {
        unsafe {
            let mut texture_binding_2d = MaybeUninit::uninit();
            self.0.gl.get_integerv(gl::TEXTURE_BINDING_2D, texture_binding_2d.as_mut_ptr());
            let texture_binding_2d = texture_binding_2d.assume_init() as u32;

            if texture_binding_2d != self.0.tex {
                self.0.gl.bind_texture(gl::TEXTURE_2D, self.0.tex);
            }
        }
    }
}

impl Clone for TextureGL {
    #[inline]
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
