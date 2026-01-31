use std::{mem::MaybeUninit, ptr, rc::Rc, slice};

use crate::gl::{self, Gl};

pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

pub struct Buffer {
    gl: Rc<Gl>,
    buffer: u32,
}

impl Buffer {
    pub fn new<T: Copy>(gl: Rc<Gl>, data: &[T], usage: BufferUsage) -> Self {
        let buffer;
        unsafe {
            let mut buffers = [MaybeUninit::uninit()];
            gl.gen_buffers(slice::from_raw_parts_mut(
                buffers.as_mut_ptr().cast(),
                buffers.len(),
            ));
            buffer = buffers[0].assume_init();

            gl.bind_buffer(gl::COPY_READ_BUFFER, buffer);
            gl.buffer_data(
                gl::COPY_READ_BUFFER,
                (data.len() * size_of::<T>()).cast_signed(),
                data.as_ptr().cast(),
                match usage {
                    BufferUsage::StreamDraw => gl::STREAM_DRAW,
                    BufferUsage::StreamRead => gl::STREAM_READ,
                    BufferUsage::StreamCopy => gl::STREAM_COPY,
                    BufferUsage::StaticDraw => gl::STATIC_DRAW,
                    BufferUsage::StaticRead => gl::STATIC_READ,
                    BufferUsage::StaticCopy => gl::STATIC_COPY,
                    BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
                    BufferUsage::DynamicRead => gl::DYNAMIC_READ,
                    BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
                },
            );
        }

        Self { gl, buffer }
    }

    pub fn new_uninit(gl: Rc<Gl>, len: usize, usage: BufferUsage) -> Self {
        let buffer;
        unsafe {
            let mut buffers = [MaybeUninit::uninit()];
            gl.gen_buffers(slice::from_raw_parts_mut(
                buffers.as_mut_ptr().cast(),
                buffers.len(),
            ));
            buffer = buffers[0].assume_init();

            gl.bind_buffer(gl::COPY_READ_BUFFER, buffer);
            gl.buffer_data(
                gl::COPY_READ_BUFFER,
                len.cast_signed(),
                ptr::null(),
                match usage {
                    BufferUsage::StreamDraw => gl::STREAM_DRAW,
                    BufferUsage::StreamRead => gl::STREAM_READ,
                    BufferUsage::StreamCopy => gl::STREAM_COPY,
                    BufferUsage::StaticDraw => gl::STATIC_DRAW,
                    BufferUsage::StaticRead => gl::STATIC_READ,
                    BufferUsage::StaticCopy => gl::STATIC_COPY,
                    BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
                    BufferUsage::DynamicRead => gl::DYNAMIC_READ,
                    BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
                },
            );
        }

        Self { gl, buffer }
    }

    pub fn copy_from_slice<T: Copy>(&self, offset: usize, slice: &[T]) {
        unsafe {
            self.gl.bind_buffer(gl::COPY_READ_BUFFER, self.buffer);
            self.gl.buffer_sub_data(
                gl::COPY_READ_BUFFER,
                offset.cast_signed(),
                (slice.len() * size_of::<T>()).cast_signed(),
                slice.as_ptr().cast(),
            );
        }
    }
}

impl Drop for Buffer {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.gl.delete_buffers(&[self.buffer]) }
    }
}
