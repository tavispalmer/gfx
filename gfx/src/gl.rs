use std::{ffi::{c_char, c_double, c_float, c_int, c_schar, c_short, c_uchar, c_uint, c_ushort, c_void, CStr}, mem::{self, MaybeUninit}, ptr};

pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = c_schar;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLubyte = c_uchar;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLsizei = c_int;
pub type GLfloat = c_float;
pub type GLclampf = c_float;
pub type GLdouble = c_double;
pub type GLclampd = c_double;

// 141
pub const UNSIGNED_SHORT: GLenum = 0x1403;
// 144
pub const FLOAT: GLenum = 0x1406;
// 155
pub const TRIANGLES: GLenum = 0x0004;
// 654
pub const NO_ERROR: GLenum = 0;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const STACK_OVERFLOW: GLenum = 0x0503;
pub const STACK_UNDERFLOW: GLenum = 0x0504;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
// 677
pub const COLOR_BUFFER_BIT: GLbitfield = 0x00004000;

pub type PFNGLCLEARCOLORPROC = unsafe extern "system" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
pub type PFNGLCLEARPROC = unsafe extern "system" fn(mask: GLbitfield);
// 786
pub type PFNGLENABLEPROC = unsafe extern "system" fn(cap: GLenum);
pub type PFNGLDISABLEPROC = unsafe extern "system" fn(cap: GLenum);
// 819
pub type PFNGLGETERRORPROC = unsafe extern "system" fn() -> GLenum;
pub type PFNGLGETSTRINGPROC = unsafe extern "system" fn(name: GLenum) -> *const GLubyte;
// 866
pub type PFNGLVIEWPORTPROC = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
// 1135
pub type PFNGLDRAWARRAYSPROC = unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei);
pub type PFNGLDRAWELEMENTSPROC = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid);

// 450
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
// 458
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
// 469
pub const STREAM_DRAW: GLenum = 0x88E0;
// 472
pub const STATIC_DRAW: GLenum = 0x88E4;
// 475
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
// 510
pub type PFNGLBINDBUFFERPROC = unsafe extern "system" fn(target: GLenum, buffer: GLuint);
pub type PFNGLDELETEBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);
pub type PFNGLGENBUFFERSPROC = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);
// 514
pub type PFNGLBUFFERDATAPROC = unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
pub type PFNGLBUFFERSUBDATAPROC = unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void);
// 546
pub type GLchar = c_char;
// 580
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;
// 608
pub const COMPILE_STATUS: GLenum = 0x8B81;
pub const LINK_STATUS: GLenum = 0x8B82;
// 611
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
// 636
pub type PFNGLATTACHSHADERPROC = unsafe extern "system" fn(program: GLuint, shader: GLuint);
// 638
pub type PFNGLCOMPILESHADERPROC = unsafe extern "system" fn(shader: GLuint);
pub type PFNGLCREATEPROGRAMPROC = unsafe extern "system" fn() -> GLuint;
pub type PFNGLCREATESHADERPROC = unsafe extern "system" fn(type_: GLenum) -> GLuint;
pub type PFNGLDELETEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLDELETESHADERPROC = unsafe extern "system" fn(shader: GLuint);
// 644
pub type PFNGLDISABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);
pub type PFNGLENABLEVERTEXATTRIBARRAYPROC = unsafe extern "system" fn(index: GLuint);
// 649
pub type PFNGLGETATTRIBLOCATIONPROC = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
pub type PFNGLGETPROGRAMIVPROC = unsafe extern "system" fn(program: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETPROGRAMINFOLOGPROC = unsafe extern "system" fn(prgoram: GLuint, buf_size: GLsizei, length: *mut GLsizei, info_log: *mut GLchar);
pub type PFNGLGETSHADERIVPROC = unsafe extern "system" fn(shader: GLuint, pname: GLenum, params: *mut GLint);
pub type PFNGLGETSHADERINFOLOGPROC = unsafe extern "system" fn(shader: GLuint, buf_size: GLsizei, length: *mut GLsizei, info_log: *mut GLchar);
// 655
pub type PFNGLGETUNIFORMLOCATIONPROC = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
// 664
pub type PFNGLLINKPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLSHADERSOURCEPROC = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
pub type PFNGLUSEPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
// 685
pub type PFNGLUNIFORMMATRIX4FVPROC = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 723
pub type PFNGLVERTEXATTRIBPOINTERPROC = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
// 968
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
// 1044
pub const FRAMEBUFFER: GLenum = 0x8D40;
// 1167
pub type PFNGLBINDFRAMEBUFFERPROC = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
// 1182
pub type PFNGLBINDVERTEXARRAYPROC = unsafe extern "system" fn(array: GLuint);
pub type PFNGLDELETEVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
pub type PFNGLGENVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
// 2545
pub const CONTEXT_LOST: GLenum = 0x0507;

#[derive(Clone)]
pub struct Gl {
    clear_color: PFNGLCLEARCOLORPROC,
    clear: PFNGLCLEARPROC,
    enable: PFNGLENABLEPROC,
    disable: PFNGLDISABLEPROC,
    get_error: PFNGLGETERRORPROC,
    get_string: PFNGLGETSTRINGPROC,
    viewport: PFNGLVIEWPORTPROC,
    draw_arrays: PFNGLDRAWARRAYSPROC,
    draw_elements: PFNGLDRAWELEMENTSPROC,

    bind_buffer: PFNGLBINDBUFFERPROC,
    delete_buffers: PFNGLDELETEBUFFERSPROC,
    gen_buffers: PFNGLGENBUFFERSPROC,
    buffer_data: PFNGLBUFFERDATAPROC,
    buffer_sub_data: PFNGLBUFFERSUBDATAPROC,
    attach_shader: PFNGLATTACHSHADERPROC,
    compile_shader: PFNGLCOMPILESHADERPROC,
    create_program: PFNGLCREATEPROGRAMPROC,
    create_shader: PFNGLCREATESHADERPROC,
    delete_program: PFNGLDELETEPROGRAMPROC,
    delete_shader: PFNGLDELETESHADERPROC,
    disable_vertex_attrib_array: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    enable_vertex_attrib_array: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    get_attrib_location: PFNGLGETATTRIBLOCATIONPROC,
    get_programiv: PFNGLGETPROGRAMIVPROC,
    get_program_info_log: PFNGLGETPROGRAMINFOLOGPROC,
    get_shaderiv: PFNGLGETSHADERIVPROC,
    get_shader_info_log: PFNGLGETSHADERINFOLOGPROC,
    get_uniform_location: PFNGLGETUNIFORMLOCATIONPROC,
    link_program: PFNGLLINKPROGRAMPROC,
    shader_source: PFNGLSHADERSOURCEPROC,
    use_program: PFNGLUSEPROGRAMPROC,
    uniform_matrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
    vertex_attrib_pointer: PFNGLVERTEXATTRIBPOINTERPROC,
    bind_framebuffer: PFNGLBINDFRAMEBUFFERPROC,
    bind_vertex_array: PFNGLBINDVERTEXARRAYPROC,
    delete_vertex_arrays: PFNGLDELETEVERTEXARRAYSPROC,
    gen_vertex_arrays: PFNGLGENVERTEXARRAYSPROC,
}
unsafe impl Send for Gl {}
unsafe impl Sync for Gl {}
impl Gl {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut f: F) -> Self {
        Self::load_erased(&mut f)
    }
    fn load_erased(f: &mut dyn FnMut(&CStr) -> *const c_void) -> Self {
        Self {
            clear_color: unsafe {
                unsafe extern "system" fn clear_color(
                    _red: GLclampf,
                    _green: GLclampf,
                    _blue: GLclampf,
                    _alpha: GLclampf,
                ) {
                    panic!("Unable to load clear_color")
                }
                let val = f(c"glClearColor");
                if val.is_null() {
                    clear_color
                } else {
                    mem::transmute(val)
                }
            },
            clear: unsafe {
                unsafe extern "system" fn clear(
                    _mask: GLbitfield,
                ) {
                    panic!("Unable to load clear")
                }
                let val = f(c"glClear");
                if val.is_null() {
                    clear
                } else {
                    mem::transmute(val)
                }
            },
            enable: unsafe {
                unsafe extern "system" fn enable(
                    _cap: GLenum,
                ) {
                    panic!("Unable to load enable")
                }
                let val = f(c"glEnable");
                if val.is_null() {
                    enable
                } else {
                    mem::transmute(val)
                }
            },
            disable: unsafe {
                unsafe extern "system" fn disable(
                    _cap: GLenum,
                ) {
                    panic!("Unable to load disable")
                }
                let val = f(c"glDisable");
                if val.is_null() {
                    disable
                } else {
                    mem::transmute(val)
                }
            },
            get_error: unsafe {
                unsafe extern "system" fn get_error() -> GLenum {
                    panic!("Unable to load get_error")
                }
                let val = f(c"glGetError");
                if val.is_null() {
                    get_error
                } else {
                    mem::transmute(val)
                }
            },
            get_string: unsafe {
                unsafe extern "system" fn get_string(
                    _name: GLenum,
                ) -> *const GLubyte {
                    panic!("Unable to load get_string")
                }
                let val = f(c"glGetString");
                if val.is_null() {
                    get_string
                } else {
                    mem::transmute(val)
                }
            },
            viewport: unsafe {
                unsafe extern "system" fn viewport(
                    _x: GLint,
                    _y: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                ) {
                    panic!("Unable to load viewport")
                }
                let val = f(c"glViewport");
                if val.is_null() {
                    viewport
                } else {
                    mem::transmute(val)
                }
            },
            draw_arrays: unsafe {
                unsafe extern "system" fn draw_arrays(
                    _mode: GLenum,
                    _first: GLint,
                    _count: GLsizei,
                ) {
                    panic!("Unable to load draw_arrays")
                }
                let val = f(c"glDrawArrays");
                if val.is_null() {
                    draw_arrays
                } else {
                    mem::transmute(val)
                }
            },
            draw_elements: unsafe {
                unsafe extern "system" fn draw_elements(
                    _mode: GLenum,
                    _count: GLsizei,
                    _type: GLenum,
                    _indices: *const GLvoid,
                ) {
                    panic!("Unable to load draw_elements")
                }
                let val = f(c"glDrawElements");
                if val.is_null() {
                    draw_elements
                } else {
                    mem::transmute(val)
                }
            },
            bind_buffer: unsafe {
                unsafe extern "system" fn bind_buffer(
                    _target: GLenum,
                    _buffer: GLuint,
                ) {
                    panic!("Unable to load bind_buffer")
                }
                let val = f(c"glBindBuffer");
                if val.is_null() {
                    bind_buffer
                } else {
                    mem::transmute(val)
                }
            },
            delete_buffers: unsafe {
                unsafe extern "system" fn delete_buffers(
                    _n: GLsizei,
                    _buffers: *const GLuint,
                ) {
                    panic!("Unable to load delete_buffers")
                }
                let val = f(c"glDeleteBuffers");
                if val.is_null() {
                    delete_buffers
                } else {
                    mem::transmute(val)
                }
            },
            gen_buffers: unsafe {
                unsafe extern "system" fn gen_buffers(
                    _n: GLsizei,
                    _buffers: *mut GLuint,
                ) {
                    panic!("Unable to load gen_buffers")
                }
                let val = f(c"glGenBuffers");
                if val.is_null() {
                    gen_buffers
                } else {
                    mem::transmute(val)
                }
            },
            buffer_data: unsafe {
                unsafe extern "system" fn buffer_data(
                    _target: GLenum,
                    _size: GLsizeiptr,
                    _data: *const c_void,
                    _usage: GLenum,
                ) {
                    panic!("Unable to load buffer_data")
                }
                let val = f(c"glBufferData");
                if val.is_null() {
                    buffer_data
                } else {
                    mem::transmute(val)
                }
            },
            buffer_sub_data: unsafe {
                unsafe extern "system" fn buffer_sub_data(
                    _target: GLenum,
                    _offset: GLintptr,
                    _size: GLsizeiptr,
                    _data: *const c_void,
                ) {
                    panic!("Unable to load buffer_sub_data")
                }
                let val = f(c"glBufferSubData");
                if val.is_null() {
                    buffer_sub_data
                } else {
                    mem::transmute(val)
                }
            },
            attach_shader: unsafe {
                unsafe extern "system" fn attach_shader(
                    _program: GLuint,
                    _shader: GLuint,
                ) {
                    panic!("Unable to load attach_shader")
                }
                let val = f(c"glAttachShader");
                if val.is_null() {
                    attach_shader
                } else {
                    mem::transmute(val)
                }
            },
            compile_shader: unsafe {
                unsafe extern "system" fn compile_shader(
                    _shader: GLuint
                ) {
                    panic!("Unable to load compile_shader")
                }
                let val = f(c"glCompileShader");
                if val.is_null() {
                    compile_shader
                } else {
                    mem::transmute(val)
                }
            },
            create_program: unsafe {
                unsafe extern "system" fn create_program() -> GLuint {
                    panic!("Unable to load create_program")
                }
                let val = f(c"glCreateProgram");
                if val.is_null() {
                    create_program
                } else {
                    mem::transmute(val)
                }
            },
            create_shader: unsafe {
                unsafe extern "system" fn create_shader(
                    _type: GLenum,
                ) -> GLuint {
                    panic!("Unable to load create_shader")
                }
                let val = f(c"glCreateShader");
                if val.is_null() {
                    create_shader
                } else {
                    mem::transmute(val)
                }
            },
            delete_program: unsafe {
                unsafe extern "system" fn delete_program(
                    _program: GLuint,
                ) {
                    panic!("Unable to load delete_program")
                }
                let val = f(c"glDeleteProgram");
                if val.is_null() {
                    delete_program
                } else {
                    mem::transmute(val)
                }
            },
            delete_shader: unsafe {
                unsafe extern "system" fn delete_shader(
                    _shader: GLuint,
                ) {
                    panic!("Unable to load delete_shader")
                }
                let val = f(c"glDeleteShader");
                if val.is_null() {
                    delete_shader
                } else {
                    mem::transmute(val)
                }
            },
            disable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn disable_vertex_attrib_array(
                    _index: GLuint,
                ) {
                    panic!("Unable to load disable_vertex_attrib_array")
                }
                let val = f(c"glDisableVertexAttribArray");
                if val.is_null() {
                    disable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            enable_vertex_attrib_array: unsafe {
                unsafe extern "system" fn enable_vertex_attrib_array(
                    _index: GLuint,
                ) {
                    panic!("Unable to load enable_vertex_attrib_array")
                }
                let val = f(c"glEnableVertexAttribArray");
                if val.is_null() {
                    enable_vertex_attrib_array
                } else {
                    mem::transmute(val)
                }
            },
            get_attrib_location: unsafe {
                unsafe extern "system" fn get_attrib_location(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_attrib_location")
                }
                let val = f(c"glGetAttribLocation");
                if val.is_null() {
                    get_attrib_location
                } else {
                    mem::transmute(val)
                }
            },
            get_programiv: unsafe {
                unsafe extern "system" fn get_programiv(
                    _program: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_programiv")
                }
                let val = f(c"glGetProgramiv");
                if val.is_null() {
                    get_programiv
                } else {
                    mem::transmute(val)
                }
            },
            get_program_info_log: unsafe {
                unsafe extern "system" fn get_program_info_log(
                    _program: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _info_log: *mut GLchar,
                ) {
                    panic!("Unable to load get_program_info_log")
                }
                let val = f(c"glGetProgramInfoLog");
                if val.is_null() {
                    get_program_info_log
                } else {
                    mem::transmute(val)
                }
            },
            get_shaderiv: unsafe {
                unsafe extern "system" fn get_shaderiv(
                    _shader: GLuint,
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_shaderiv")
                }
                let val = f(c"glGetShaderiv");
                if val.is_null() {
                    get_shaderiv
                } else {
                    mem::transmute(val)
                }
            },
            get_shader_info_log: unsafe {
                unsafe extern "system" fn get_shader_info_log(
                    _shader: GLuint,
                    _buf_size: GLsizei,
                    _length: *mut GLsizei,
                    _info_log: *mut GLchar,
                ) {
                    panic!("Unable to load get_shader_info_log")
                }
                let val = f(c"glGetShaderInfoLog");
                if val.is_null() {
                    get_shader_info_log
                } else {
                    mem::transmute(val)
                }
            },
            get_uniform_location: unsafe {
                unsafe extern "system" fn get_uniform_location(
                    _program: GLuint,
                    _name: *const GLchar,
                ) -> GLint {
                    panic!("Unable to load get_uniform_location")
                }
                let val = f(c"glGetUniformLocation");
                if val.is_null() {
                    get_uniform_location
                } else {
                    mem::transmute(val)
                }
            },
            link_program: unsafe {
                unsafe extern "system" fn link_program(
                    _program: GLuint,
                ) {
                    panic!("Unable to load link_program")
                }
                let val = f(c"glLinkProgram");
                if val.is_null() {
                    link_program
                } else {
                    mem::transmute(val)
                }
            },
            shader_source: unsafe {
                unsafe extern "system" fn shader_source(
                    _shader: GLuint,
                    _count: GLsizei,
                    _string: *const *const GLchar,
                    _length: *const GLint,
                ) {
                    panic!("Unable to load shader_source")
                }
                let val = f(c"glShaderSource");
                if val.is_null() {
                    shader_source
                } else {
                    mem::transmute(val)
                }
            },
            use_program: unsafe {
                unsafe extern "system" fn use_program(
                    _program: GLuint,
                ) {
                    panic!("Unable to load use_program")
                }
                let val = f(c"glUseProgram");
                if val.is_null() {
                    use_program
                } else {
                    mem::transmute(val)
                }
            },
            uniform_matrix4fv: unsafe {
                unsafe extern "system" fn uniform_matrix4fv(
                    _location: GLint,
                    _count: GLsizei,
                    _transpose: GLboolean,
                    _value: *const GLfloat,
                ) {
                    panic!("Unable to load uniform_matrix4fv")
                }
                let val = f(c"glUniformMatrix4fv");
                if val.is_null() {
                    uniform_matrix4fv
                } else {
                    mem::transmute(val)
                }
            },
            vertex_attrib_pointer: unsafe {
                unsafe extern "system" fn vertex_attrib_pointer(
                    _index: GLuint,
                    _size: GLint,
                    _type: GLenum,
                    _normalized: GLboolean,
                    _stride: GLsizei,
                    _pointer: *const c_void,
                ) {
                    panic!("Unable to load vertex_attrib_pointer")
                }
                let val = f(c"glVertexAttribPointer");
                if val.is_null() {
                    vertex_attrib_pointer
                } else {
                    mem::transmute(val)
                }
            },
            bind_framebuffer: unsafe {
                unsafe extern "system" fn bind_framebuffer(
                    _target: GLenum,
                    _framebuffer: GLuint,
                ) {
                    panic!("Unable to load bind_framebuffer")
                }
                let val = f(c"glBindFramebuffer");
                if val.is_null() {
                    bind_framebuffer
                } else {
                    mem::transmute(val)
                }
            },
            bind_vertex_array: unsafe {
                 unsafe extern "system" fn bind_vertex_array(
                    _array: GLuint,
                 ) {
                    panic!("Unable to load bind_vertex_array")
                 }
                 let val = f(c"glBindVertexArray");
                 if val.is_null() {
                    bind_vertex_array
                 } else {
                    mem::transmute(val)
                 }
            },
            delete_vertex_arrays: unsafe {
                unsafe extern "system" fn delete_vertex_arrays(
                    _n: GLsizei,
                    _arrays: *const GLuint,
                ) {
                    panic!("Unable to load delete_vertex_arrays")
                }
                let val = f(c"glDeleteVertexArrays");
                if val.is_null() {
                    delete_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
            gen_vertex_arrays: unsafe {
                unsafe extern "system" fn gen_vertex_arrays(
                    _n: GLsizei,
                    _arrays: *mut GLuint,
                ) {
                    panic!("Unable to load gen_vertex_arrays")
                }
                let val = f(c"glGenVertexArrays");
                if val.is_null() {
                    gen_vertex_arrays
                } else {
                    mem::transmute(val)
                }
            },
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    unsafe fn check_error(&self, func: &str) {
        unsafe {
            let error = self.get_error();
            match error {
                NO_ERROR => {},
                INVALID_ENUM => panic!(concat!("{}: ", stringify!(INVALID_ENUM)), func),
                INVALID_VALUE => panic!(concat!("{}: ", stringify!(INVALID_VALUE)), func),
                INVALID_OPERATION => panic!(concat!("{}: ", stringify!(INVALID_OPERATION)), func),
                STACK_OVERFLOW => panic!(concat!("{}: ", stringify!(STACK_OVERFLOW)), func),
                STACK_UNDERFLOW => panic!(concat!("{}: ", stringify!(STACK_UNDERFLOW)), func),
                OUT_OF_MEMORY => panic!(concat!("{}: ", stringify!(OUT_OF_MEMORY)), func),
                INVALID_FRAMEBUFFER_OPERATION => panic!(concat!("{}: ", stringify!(INVALID_FRAMEBUFFER_OPERATION)), func),
                CONTEXT_LOST => panic!(concat!("{}: ", stringify!(CONTEXT_LOST)), func),
                _ => panic!("{func}: unknown error")
            }
        }
    }

    #[inline]
    pub unsafe fn clear_color(&self, red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
        unsafe {
            (self.clear_color)(red, green, blue, alpha);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(clear_color));
        }
    }
    #[inline]
    pub unsafe fn clear(&self, mask: GLbitfield) {
        unsafe {
            (self.clear)(mask);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(clear));
        }
    }
    #[inline]
    pub unsafe fn enable(&self, cap: GLenum) {
        unsafe {
            (self.enable)(cap);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(enable));
        }
    }
    #[inline]
    pub unsafe fn disable(&self, cap: GLenum) {
        unsafe {
            (self.disable)(cap);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(disable));
        }
    }
    #[inline]
    pub unsafe fn get_error(&self) -> GLenum {
        unsafe {
            (self.get_error)()
        }
    }
    #[inline]
    pub unsafe fn get_string(&self, name: GLenum) -> &CStr {
        unsafe {
            CStr::from_ptr((self.get_string)(name).cast())
        }
    }
    #[inline]
    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe {
            (self.viewport)(x, y, width, height);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(viewport));
        }
    }
    #[inline]
    pub unsafe fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        unsafe {
            (self.draw_arrays)(mode, first, count);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(draw_arrays));
        }
    }
    #[inline]
    pub unsafe fn draw_elements(&self, mode: GLenum, count: GLsizei, type_: GLenum, indices: usize) {
        unsafe {
            (self.draw_elements)(mode, count, type_, indices as *const GLvoid);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(draw_elements));
        }
    }
    #[inline]
    pub unsafe fn bind_buffer(&self, target: GLenum, buffer: GLuint) {
        unsafe {
            (self.bind_buffer)(target, buffer);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(bind_buffer));
        }
    }
    #[inline]
    pub unsafe fn delete_buffers(&self, buffers: &[GLuint]) {
        unsafe {
            (self.delete_buffers)(buffers.len() as GLsizei, buffers.as_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(delete_buffers));
        }
    }
    #[inline]
    pub unsafe fn gen_buffers(&self, buffers: &mut [GLuint]) {
        unsafe {
            (self.gen_buffers)(buffers.len() as GLsizei, buffers.as_mut_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(gen_buffers));
        }
    }
    #[inline]
    pub unsafe fn buffer_data(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum) {
        unsafe {
            (self.buffer_data)(target, size, data, usage);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(buffer_data));
        }
    }
    #[inline]
    pub unsafe fn buffer_sub_data(&self, target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const c_void) {
        unsafe {
            (self.buffer_sub_data)(target, offset, size, data);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(buffer_sub_data));
        }
    }
    #[inline]
    pub unsafe fn attach_shader(&self, program: GLuint, shader: GLuint) {
        unsafe {
            (self.attach_shader)(program, shader);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(attach_shader));
        }
    }
    #[inline]
    pub unsafe fn compile_shader(&self, shader: GLuint) {
        unsafe {
            (self.compile_shader)(shader);
            #[cfg(debug_assertions)] {
                self.check_error(stringify!(compile_shader));
                let mut compile_status = MaybeUninit::uninit();
                self.get_shaderiv(shader, COMPILE_STATUS, compile_status.as_mut_ptr());
                let compile_status = compile_status.assume_init();
                if compile_status == 0 {
                    let mut info_log_length = MaybeUninit::uninit();
                    self.get_shaderiv(shader, INFO_LOG_LENGTH, info_log_length.as_mut_ptr());
                    let info_log_length = info_log_length.assume_init();
                    let mut info_log: Vec<u8> = Vec::with_capacity(info_log_length as usize);
                    self.get_shader_info_log(shader, info_log_length, ptr::null_mut(), info_log.as_mut_ptr().cast());
                    info_log.set_len(info_log_length as usize - 1);
                    panic!("{}", str::from_utf8(&info_log).unwrap());
                }
            }
        }
    }
    #[inline]
    pub unsafe fn create_program(&self) -> GLuint {
        unsafe {
            let result = (self.create_program)();
            #[cfg(debug_assertions)]
            self.check_error(stringify!(create_program));
            result
        }
    }
    #[inline]
    pub unsafe fn create_shader(&self, type_: GLenum) -> GLuint {
        unsafe {
            let result = (self.create_shader)(type_);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(create_shader));
            result
        }
    }
    #[inline]
    pub unsafe fn delete_program(&self, program: GLuint) {
        unsafe {
            (self.delete_program)(program);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(delete_program));
        }
    }
    #[inline]
    pub unsafe fn delete_shader(&self, shader: GLuint) {
        unsafe {
            (self.delete_shader)(shader);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(delete_shader));
        }
    }
    #[inline]
    pub unsafe fn disable_vertex_attrib_array(&self, index: GLuint) {
        unsafe {
            (self.disable_vertex_attrib_array)(index);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(disable_vertex_attrib_array));
        }
    }
    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, index: GLuint) {
        unsafe {
            (self.enable_vertex_attrib_array)(index);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(enable_vertex_attrib_array));
        }
    }
    #[inline]
    pub unsafe fn get_attrib_location(&self, program: GLuint, name: &CStr) -> GLint {
        unsafe {
            let result = (self.get_attrib_location)(program, name.as_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_attrib_location));
            result
        }
    }
    #[inline]
    pub unsafe fn get_programiv(&self, program: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe {
            (self.get_programiv)(program, pname, params);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_programiv));
        }
    }
    #[inline]
    pub unsafe fn get_program_info_log(&self, program: GLuint, buf_size: GLsizei, length: *mut GLsizei, info_log: *mut GLchar) {
        unsafe {
            (self.get_program_info_log)(program, buf_size, length, info_log);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_program_info_log));
        }
    }
    #[inline]
    pub unsafe fn get_shaderiv(&self, shader: GLuint, pname: GLenum, params: *mut GLint) {
        unsafe {
            (self.get_shaderiv)(shader, pname, params);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_program_info_log));
        }
    }
    #[inline]
    pub unsafe fn get_shader_info_log(&self, shader: GLuint, buf_size: GLsizei, length: *mut GLsizei, info_log: *mut GLchar) {
        unsafe {
            (self.get_shader_info_log)(shader, buf_size, length, info_log);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_shader_info_log));
        }
    }
    #[inline]
    pub unsafe fn get_uniform_location(&self, program: GLuint, name: &CStr) -> GLint {
        unsafe {
            let result = (self.get_uniform_location)(program, name.as_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(get_uniform_location));
            result
        }
    }
    #[inline]
    pub unsafe fn link_program(&self, program: GLuint) {
        unsafe {
            (self.link_program)(program);
            #[cfg(debug_assertions)] {
                self.check_error(stringify!(link_program));
                let mut link_status = MaybeUninit::uninit();
                self.get_programiv(program, LINK_STATUS, link_status.as_mut_ptr());
                let link_status = link_status.assume_init();
                if link_status == 0 {
                    let mut info_log_length = MaybeUninit::uninit();
                    self.get_programiv(program, INFO_LOG_LENGTH, info_log_length.as_mut_ptr());
                    let info_log_length = info_log_length.assume_init();
                    let mut info_log: Vec<u8> = Vec::with_capacity(info_log_length as usize);
                    self.get_program_info_log(program, info_log_length, ptr::null_mut(), info_log.as_mut_ptr().cast());
                    info_log.set_len(info_log_length as usize - 1);
                    panic!("{}", str::from_utf8(&info_log).unwrap());
                }
            }
        }
    }
    #[inline]
    pub unsafe fn shader_source(&self, shader: GLuint, string: &str) {
        unsafe {
            (self.shader_source)(shader, 1, &string.as_ptr().cast(), &(string.len() as GLint));
            #[cfg(debug_assertions)]
            self.check_error(stringify!(shader_source));
        }
    }
    #[inline]
    pub unsafe fn use_program(&self, program: GLuint) {
        unsafe {
            (self.use_program)(program);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(use_program));
        }
    }
    #[inline]
    pub unsafe fn uniform_matrix4fv(&self, location: GLint, count: GLsizei, transpose: bool, value: *const GLfloat) {
        unsafe {
            (self.uniform_matrix4fv)(location, count, transpose as GLboolean, value);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(uniform_matrix4fv));
        }
    }
    #[inline]
    pub unsafe fn vertex_attrib_pointer(&self, index: GLuint, size: GLint, type_: GLenum, normalized: bool, stride: GLsizei, pointer: usize) {
        unsafe {
            (self.vertex_attrib_pointer)(index, size, type_, normalized as GLboolean, stride, pointer as *const c_void);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(vertex_attrib_pointer));
        }
    }
    #[inline]
    pub unsafe fn bind_framebuffer(&self, target: GLenum, framebuffer: GLuint) {
        unsafe {
            (self.bind_framebuffer)(target, framebuffer);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(bind_framebuffer));
        }
    }
    #[inline]
    pub unsafe fn bind_vertex_array(&self, array: GLuint) {
        unsafe {
            (self.bind_vertex_array)(array);
            #[cfg(debug_assertions)]
            self.check_error(stringify!(bind_vertex_array));
        }
    }
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, arrays: &[GLuint]) {
        unsafe {
            (self.delete_vertex_arrays)(arrays.len() as GLsizei, arrays.as_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(delete_vertex_arrays));
        }
    }
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, arrays: &mut [GLuint]) {
        unsafe {
            (self.gen_vertex_arrays)(arrays.len() as GLsizei, arrays.as_mut_ptr());
            #[cfg(debug_assertions)]
            self.check_error(stringify!(gen_vertex_arrays));
        }
    }
}
