use std::{mem::MaybeUninit, num::NonZero, ptr, rc::Rc, slice};

use crate::{gl, GL};

pub struct Buffer {
    // vtable
    gl: Rc<GL>,

    buf: NonZero<u32>,
    size: usize,
}

impl Buffer {
    #[inline]
    pub const fn size(&self) -> usize {
        self.size
    }

    pub fn new(gl: Rc<GL>, size: usize, usage: u32) -> Self {
        unsafe {
            let mut buf = MaybeUninit::uninit();
            gl.gen_buffers(slice::from_raw_parts_mut(
                buf.as_mut_ptr(),
                1,
            ));
            let buf = NonZero::new(buf.assume_init()).unwrap();

            gl.bind_buffer(gl::COPY_WRITE_BUFFER, buf.get());
            gl.buffer_data(
                gl::COPY_WRITE_BUFFER,
                size.cast_signed(),
                ptr::null(),
                usage,
            );

            Self {
                gl,
                buf,
                size,
            }
        }
    }

    pub fn bind(&self, target: u32) {
        unsafe {
            let mut binding = MaybeUninit::uninit();
            self.gl.get_integerv(
                match target {
                    gl::ARRAY_BUFFER => gl::ARRAY_BUFFER_BINDING,
                    gl::ELEMENT_ARRAY_BUFFER => gl::ELEMENT_ARRAY_BUFFER_BINDING,
                    gl::COPY_READ_BUFFER => gl::COPY_READ_BUFFER_BINDING,
                    gl::COPY_WRITE_BUFFER => gl::COPY_WRITE_BUFFER_BINDING,
                    _ => panic!("bind: Unknown target"),
                },
                binding.as_mut_ptr(),
            );
            let binding = binding.assume_init() as u32;
            if binding != self.buf.get() {
                self.gl.bind_buffer(target, self.buf.get());
            }
        }
    }

    pub fn copy_from_slice(&self, src: &[u8], offset: usize) {
        self.bind(gl::COPY_WRITE_BUFFER);
        unsafe {
            self.gl.buffer_sub_data(
                gl::COPY_WRITE_BUFFER,
                offset.cast_signed(),
                src.len().cast_signed(),
                src.as_ptr().cast(),
            );
        }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffers(&[self.buf.get()]);
        }
    }
}
