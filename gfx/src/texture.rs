use std::{path::Path, rc::Rc};

use crate::GL;

pub struct TextureGL {
    // vtable
    gl: Rc<GL>,
    tex: u32,
}

impl TextureGL {
    pub fn open<P: AsRef<Path>>(gl: Rc<GL>, path: P) {
        Self::open_erased(gl, path.as_ref())
    }

    fn open_erased(gl: Rc<GL>, path: &Path) {
        // here we use stbi
        
    }
}