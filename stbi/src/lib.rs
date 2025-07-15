mod ffi;

use std::{ffi::{c_char, c_int, c_uint, c_void}, fs::File, io::{Error, Read, Result, Seek, SeekFrom}, mem::MaybeUninit, path::Path, ptr::NonNull, slice};

use ffi::*;

#[derive(Clone, Copy, Debug)]
pub enum Channels {
    Grey = STBI_grey as isize,
    GreyAlpha = STBI_grey_alpha as isize,
    Rgb = STBI_rgb as isize,
    RgbAlpha = STBI_rgb_alpha as isize,
}

pub struct Image {
    data: NonNull<u8>,
    x: usize,
    y: usize,
    channels_in_file: Channels,
}

impl Image {
    pub const fn x(&self) -> usize {
        self.x
    }

    pub const fn y(&self) -> usize {
        self.y
    }

    pub const fn channels_in_file(&self) -> Channels {
        self.channels_in_file
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_ptr()
    }
}

impl Drop for Image {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            stbi_image_free(self.data.as_ptr() as *mut c_void)
        }
    }
}

pub fn load<P: AsRef<Path>>(filename: P, desired_channels: Option<Channels>) -> Result<Image> {
    load_erased(filename.as_ref(), desired_channels)
}

fn load_erased(filename: &Path, desired_channels: Option<Channels>) -> Result<Image> {
    // callbacks
    struct User {
        file: File,
        error: Option<Error>,
        eof: bool,
    }
    extern "C" fn read(user: *mut c_void, data: *mut c_char, size: c_int) -> c_int {
        let user = unsafe { &mut *(user as *mut User) };
        if user.error.is_none() {
            let buf = unsafe { slice::from_raw_parts_mut(data as *mut u8, size as usize) };
            match user.file.read(buf) {
                Ok(len) => {
                    if len == 0 {
                        user.eof = true;
                    }
                    len as c_int
                }
                Err(error) => {
                    user.error = Some(error);
                    0
                }
            }
        } else {
            0
        }
    }
    extern "C" fn skip(user: *mut c_void, n: c_int) {
        let user = unsafe { &mut *(user as *mut User) };
        if user.error.is_none() {
            match user.file.seek(SeekFrom::Current(n as i64)) {
                Ok(_) => {
                    // set eof
                    let mut buf = [0];
                    match user.file.read(&mut buf) {
                        Ok(len) => {
                            if len == 0 {
                                user.eof = true;
                            } else {
                                match user.file.seek(SeekFrom::Current(-1)) {
                                    Ok(_) => {},
                                    Err(error) => user.error = Some(error),
                                }
                            }
                        },
                        Err(error) => user.error = Some(error),
                    }
                }
                Err(error) => user.error = Some(error),
            }
        }
    }
    extern "C" fn eof(user: *mut c_void) -> c_int {
        let user = unsafe { &mut *(user as *mut User) };
        user.eof as c_int
    }
    static CLBK: stbi_io_callbacks = stbi_io_callbacks {
        read: Some(read),
        skip: Some(skip),
        eof: Some(eof),
    };

    let mut user = User {
        file: File::open(filename)?,
        error: None,
        eof: false,
    };
    let mut x;
    let mut y;
    let mut channels_in_file;
    let data;
    unsafe {
        x = MaybeUninit::uninit();
        y = MaybeUninit::uninit();
        channels_in_file = MaybeUninit::uninit();
        data = stbi_load_from_callbacks(
            &raw const CLBK,
            &raw mut user as *mut c_void,
            x.as_mut_ptr(),
            y.as_mut_ptr(),
            channels_in_file.as_mut_ptr(),
            match desired_channels {
                None => STBI_default,
                Some(desired_channels) => match desired_channels {
                    Channels::Grey => STBI_grey,
                    Channels::GreyAlpha => STBI_grey_alpha,
                    Channels::Rgb => STBI_rgb,
                    Channels::RgbAlpha => STBI_rgb_alpha,
                },
            } as i32,
        );
        if let Some(error) = user.error {
            Err(error)
        } else {
            Ok(Image {
                data: NonNull::new(data).unwrap(),
                x: x.assume_init() as usize,
                y: y.assume_init() as usize,
                channels_in_file: match channels_in_file.assume_init() as c_uint {
                    STBI_grey => Channels::Grey,
                    STBI_grey_alpha => Channels::GreyAlpha,
                    STBI_rgb => Channels::Rgb,
                    STBI_rgb_alpha => Channels::RgbAlpha,
                    _ => unreachable!("load: Unknown value for channels_in_file"),
                },
            })
        }
    }
}