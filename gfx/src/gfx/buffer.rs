use std::{alloc::Layout, mem::{self, MaybeUninit}, ptr, rc::Rc, slice};

use crate::{gl, GL};

pub struct Buffer {
    // vtable
    gl: Rc<GL>,

    buf: u32,
    len: usize,
}

impl Buffer {
    #[inline]
    pub const fn id(&self) -> u32 {
        self.buf
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn new(gl: Rc<GL>, layout: Layout) -> Self {
        let len = layout.size();
        if len != 0 {
            unsafe {
                let mut buf = MaybeUninit::uninit();
                gl.gen_buffers(slice::from_raw_parts_mut(
                    buf.as_mut_ptr(),
                    1,
                ));
                let buf = buf.assume_init();
                let len = layout.size();

                gl.bind_buffer(gl::COPY_WRITE_BUFFER, buf);
                gl.buffer_data(
                    gl::COPY_WRITE_BUFFER,
                    len.cast_signed(),
                    ptr::null(),
                    gl::STREAM_DRAW,
                );

                Self {
                    gl,
                    buf,
                    len,
                }
            }
        } else {
            Self {
                gl,
                buf: 0,
                len,
            }
        }
    }

    pub unsafe fn copy(&self, src: *const u8, dst: usize, count: usize) {
        unsafe {
            let mut copy_write_buffer_binding = MaybeUninit::uninit();
            self.gl.get_integerv(
                gl::COPY_WRITE_BUFFER_BINDING,
                copy_write_buffer_binding.as_mut_ptr(),
            );
            let copy_write_buffer_binding = copy_write_buffer_binding.assume_init() as u32;
            if copy_write_buffer_binding != self.buf {
                self.gl.bind_buffer(gl::COPY_WRITE_BUFFER, self.buf);
            }

            self.gl.buffer_sub_data(
                gl::COPY_WRITE_BUFFER,
                dst.cast_signed(),
                count.cast_signed(),
                src.cast(),
            );
        }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        if self.buf != 0 {
            unsafe {
                self.gl.delete_buffers(&[self.buf]);
            }
        }
    }
}
