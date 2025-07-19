use std::{ffi::*, mem};

// 116
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
pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

// 139
pub const FALSE: GLboolean = 0;
pub const TRUE: GLboolean = 1;
// 144
pub const UNSIGNED_BYTE: GLenum = 0x1401;
// 146
pub const UNSIGNED_SHORT: GLenum = 0x1403;
// 149
pub const FLOAT: GLenum = 0x1406;
// 160
pub const TRIANGLES: GLenum = 0x0004;
// 267
pub const DEPTH_TEST: GLenum = 0x0B71;
// 446
pub const RED: GLenum = 0x1903;
// 468
pub const RGB: GLenum = 0x1907;
pub const RGBA: GLenum = 0x1908;
// 611
pub const TEXTURE_2D: GLenum = 0x0DE1;
// 614
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
// 644
pub const NEAREST: GLenum = 0x2600;
// 659
pub const NO_ERROR: GLenum = 0;
pub const INVALID_ENUM: GLenum = 0x0500;
pub const INVALID_VALUE: GLenum = 0x0501;
pub const INVALID_OPERATION: GLenum = 0x0502;
pub const STACK_OVERFLOW: GLenum = 0x0503;
pub const STACK_UNDERFLOW: GLenum = 0x0504;
pub const OUT_OF_MEMORY: GLenum = 0x0505;
// 676
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
// 678
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
// 682
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
// 697
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
// 745
pub type PFNGLCLEARCOLORPROC = unsafe extern "system" fn(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
// 747
pub type PFNGLCLEARPROC = unsafe extern "system" fn(mask: GLbitfield);
// 791
pub type PFNGLENABLEPROC = unsafe extern "system" fn(cap: GLenum);
// 793
pub type PFNGLDISABLEPROC = unsafe extern "system" fn(cap: GLenum);
// 809
pub type PFNGLGETINTEGERVPROC = unsafe extern "system" fn(pname: GLenum, params: *mut GLint);
// 824
pub type PFNGLGETERRORPROC = unsafe extern "system" fn() -> GLenum;
// 871
pub type PFNGLVIEWPORTPROC = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
// 1142
pub type PFNGLDRAWELEMENTSPROC = unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid);
// 1265
pub type PFNGLTEXPARAMETERIPROC = unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint);
// 1289
pub type PFNGLTEXIMAGE2DPROC = unsafe extern "system" fn(target: GLenum, level: GLint, internal_format: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const GLvoid);
// 1302
pub type PFNGLGENTEXTURESPROC = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);
// 1304
pub type PFNGLDELETETEXTURESPROC = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);
// 1306
pub type PFNGLBINDTEXTUREPROC = unsafe extern "system" fn(target: GLenum, texture: GLuint);

// glext.h
// 450
pub type GLsizeiptr = isize;
pub type GLintptr = isize;
// 458
pub const ARRAY_BUFFER: GLenum = 0x8892;
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
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
// 517
pub type PFNGLMAPBUFFERPROC = unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut c_void;
pub type PFNGLUNMAPBUFFERPROC = unsafe extern "system" fn(target: GLenum) -> GLboolean;
// 546
pub type GLchar = c_char;
// 580
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
pub const VERTEX_SHADER: GLenum = 0x8B31;
// 620
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
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
// 655
pub type PFNGLGETUNIFORMLOCATIONPROC = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;
// 664
pub type PFNGLLINKPROGRAMPROC = unsafe extern "system" fn(program: GLuint);
pub type PFNGLSHADERSOURCEPROC = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
pub type PFNGLUSEPROGRAMPRPOC = unsafe extern "system" fn(program: GLuint);
// 685
pub type PFNGLUNIFORMMATRIX4FVPROC = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
// 723
pub type PFNGLVERTEXATTRIBPOINTERPROC = unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const c_void);
// 968
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
// 1044
pub const FRAMEBUFFER: GLenum = 0x8D40;
// 1076
pub const RG: GLenum = 0x8227;
// 1167
pub type PFNGLBINDFRAMEBUFFERPROC = unsafe extern "system" fn(target: GLenum, framebuffer: GLuint);
// 1182
pub type PFNGLBINDVERTEXARRAYPROC = unsafe extern "system" fn(array: GLuint);
pub type PFNGLDELETEVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);
pub type PFNGLGENVERTEXARRAYSPROC = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);
// 1302
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
// 1341
pub type PFNGLCOPYBUFFERSUBDATAPROC = unsafe extern "system" fn(read_target: GLenum, write_target: GLenum, read_offset: GLintptr, write_offset: GLintptr, size: GLsizeiptr);
// 2008
pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
// 2545
pub const CONTEXT_LOST: GLenum = 0x0507;

pub struct GL {
    clear_color: PFNGLCLEARCOLORPROC,
    clear: PFNGLCLEARPROC,
    enable: PFNGLENABLEPROC,
    disable: PFNGLDISABLEPROC,
    get_integerv: PFNGLGETINTEGERVPROC,
    get_error: PFNGLGETERRORPROC,
    viewport: PFNGLVIEWPORTPROC,
    draw_elements: PFNGLDRAWELEMENTSPROC,
    tex_parameteri: PFNGLTEXPARAMETERIPROC,
    tex_image_2d: PFNGLTEXIMAGE2DPROC,
    gen_textures: PFNGLGENTEXTURESPROC,
    delete_textures: PFNGLDELETETEXTURESPROC,
    bind_texture: PFNGLBINDTEXTUREPROC,
    // glext.h
    bind_buffer: PFNGLBINDBUFFERPROC,
    delete_buffers: PFNGLDELETEBUFFERSPROC,
    gen_buffers: PFNGLGENBUFFERSPROC,
    buffer_data: PFNGLBUFFERDATAPROC,
    buffer_sub_data: PFNGLBUFFERSUBDATAPROC,
    map_buffer: PFNGLMAPBUFFERPROC,
    unmap_buffer: PFNGLUNMAPBUFFERPROC,
    attach_shader: PFNGLATTACHSHADERPROC,
    compile_shader: PFNGLCOMPILESHADERPROC,
    create_program: PFNGLCREATEPROGRAMPROC,
    create_shader: PFNGLCREATESHADERPROC,
    delete_program: PFNGLDELETEPROGRAMPROC,
    delete_shader: PFNGLDELETESHADERPROC,
    disable_vertex_attrib_array: PFNGLDISABLEVERTEXATTRIBARRAYPROC,
    enable_vertex_attrib_array: PFNGLENABLEVERTEXATTRIBARRAYPROC,
    get_attrib_location: PFNGLGETATTRIBLOCATIONPROC,
    get_uniform_location: PFNGLGETUNIFORMLOCATIONPROC,
    link_program: PFNGLLINKPROGRAMPROC,
    shader_source: PFNGLSHADERSOURCEPROC,
    use_program: PFNGLUSEPROGRAMPRPOC,
    uniform_matrix4fv: PFNGLUNIFORMMATRIX4FVPROC,
    vertex_attrib_pointer: PFNGLVERTEXATTRIBPOINTERPROC,
    bind_framebuffer: PFNGLBINDFRAMEBUFFERPROC,
    bind_vertex_array: PFNGLBINDVERTEXARRAYPROC,
    delete_vertex_arrays: PFNGLDELETEVERTEXARRAYSPROC,
    gen_vertex_arrays: PFNGLGENVERTEXARRAYSPROC,
    copy_buffer_sub_data: PFNGLCOPYBUFFERSUBDATAPROC,
}

impl GL {
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
            get_integerv: unsafe {
                unsafe extern "system" fn get_integerv(
                    _pname: GLenum,
                    _params: *mut GLint,
                ) {
                    panic!("Unable to load get_integerv")
                }
                let val = f(c"glGetIntegerv");
                if val.is_null() {
                    get_integerv
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
            tex_parameteri: unsafe {
                unsafe extern "system" fn tex_parameteri(
                    _target: GLenum,
                    _pname: GLenum,
                    _param: GLint,
                ) {
                    panic!("Unable to load tex_parameteri")
                }
                let val = f(c"glTexParameteri");
                if val.is_null() {
                    tex_parameteri
                } else {
                    mem::transmute(val)
                }
            },
            tex_image_2d: unsafe {
                unsafe extern "system" fn tex_image_2d(
                    _target: GLenum,
                    _level: GLint,
                    _internal_format: GLint,
                    _width: GLsizei,
                    _height: GLsizei,
                    _border: GLint,
                    _format: GLenum,
                    _type: GLenum,
                    _pixels: *const GLvoid,
                ) {
                    panic!("Unable to load tex_image_2d")
                }
                let val = f(c"glTexImage2D");
                if val.is_null() {
                    tex_image_2d
                } else {
                    mem::transmute(val)
                }
            },
            gen_textures: unsafe {
                unsafe extern "system" fn gen_textures(
                    _n: GLsizei,
                    _textures: *mut GLuint,
                ) {
                    panic!("Unable to load gen_textures")
                }
                let val = f(c"glGenTextures");
                if val.is_null() {
                    gen_textures
                } else {
                    mem::transmute(val)
                }
            },
            delete_textures: unsafe {
                unsafe extern "system" fn delete_textures(
                    _n: GLsizei,
                    _textures: *const GLuint,
                ) {
                    panic!("Unable to load delete_textures")
                }
                let val = f(c"glDeleteTextures");
                if val.is_null() {
                    delete_textures
                } else {
                    mem::transmute(val)
                }
            },
            bind_texture: unsafe {
                unsafe extern "system" fn bind_texture(
                    _target: GLenum,
                    _texture: GLuint,
                ) {
                    panic!("Unable to load bind_texture")
                }
                let val = f(c"glBindTexture");
                if val.is_null() {
                    bind_texture
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
            map_buffer: unsafe {
                unsafe extern "system" fn map_buffer(
                    _target: GLenum,
                    _access: GLenum,
                ) -> *mut c_void {
                    panic!("Unable to load map_buffer")
                }
                let val = f(c"glMapBuffer");
                if val.is_null() {
                    map_buffer
                } else {
                    mem::transmute(val)
                }
            },
            unmap_buffer: unsafe {
                unsafe extern "system" fn unmap_buffer(
                    _target: GLenum,
                ) -> GLboolean {
                    panic!("Unable to load unmap_buffer")
                }
                let val = f(c"glUnmapBuffer");
                if val.is_null() {
                    unmap_buffer
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
                    _shader: GLuint,
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
            copy_buffer_sub_data: unsafe {
                unsafe extern "system" fn copy_buffer_sub_data(
                    _read_target: GLenum,
                    _write_target: GLenum,
                    _read_offset: GLintptr,
                    _write_offset: GLintptr,
                    _size: GLsizeiptr,
                ) {
                    panic!("Unable to load copy_buffer_sub_data")
                }
                let val = f(c"glCopyBufferSubData");
                if val.is_null() {
                    copy_buffer_sub_data
                } else {
                    mem::transmute(val)
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    unsafe fn check_error(&self, name: &str) {
        unsafe {
            match self.get_error() {
                NO_ERROR => {},
                INVALID_ENUM => panic!("{}: INVALID_ENUM", name),
                INVALID_VALUE => panic!("{}: INVALID_VALUE", name),
                INVALID_OPERATION => panic!("{}: INVALID_OPERATION", name),
                STACK_OVERFLOW => panic!("{}: STACK_OVERFLOW", name),
                STACK_UNDERFLOW => panic!("{}: STACK_UNDERFLOW", name),
                OUT_OF_MEMORY => panic!("{}: OUT_OF_MEMORY", name),
                INVALID_FRAMEBUFFER_OPERATION => panic!("{}: INVALID_FRAMEBUFFER_OPERATION", name),
                CONTEXT_LOST => panic!("{}: CONTEXT_LOST", name),
                _ => panic!("{}: Unknown error", name),
            }
        }
    }

    #[inline]
    pub unsafe fn clear_color(
        &self,
        red: GLclampf,
        green: GLclampf,
        blue: GLclampf,
        alpha: GLclampf,
    ) {
        unsafe {
            (self.clear_color)(
                red,
                green,
                blue,
                alpha,
            );
            #[cfg(debug_assertions)]
            self.check_error("clear_color");
        }
    }

    #[inline]
    pub unsafe fn clear(
        &self,
        mask: GLbitfield,
    ) {
        unsafe {
            (self.clear)(
                mask,
            );
            #[cfg(debug_assertions)]
            self.check_error("clear");
        }
    }

    #[inline]
    pub unsafe fn enable(
        &self,
        cap: GLenum,
    ) {
        unsafe {
            (self.enable)(
                cap,
            );
            #[cfg(debug_assertions)]
            self.check_error("enable");
        }
    }

    #[inline]
    pub unsafe fn disable(
        &self,
        cap: GLenum,
    ) {
        unsafe {
            (self.disable)(
                cap,
            );
            #[cfg(debug_assertions)]
            self.check_error("disable");
        }
    }

    #[inline]
    pub unsafe fn get_integerv(
        &self,
        pname: GLenum,
        params: *mut GLint,
    ) {
        unsafe {
            (self.get_integerv)(
                pname,
                params,
            );
            #[cfg(debug_assertions)]
            self.check_error("get_integerv");
        }
    }

    #[inline]
    pub unsafe fn get_error(
        &self,
    ) -> GLenum {
        unsafe {
            (self.get_error)()
        }
    }

    #[inline]
    pub unsafe fn viewport(
        &self,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    ) {
        unsafe {
            (self.viewport)(
                x,
                y,
                width,
                height,
            );
            #[cfg(debug_assertions)]
            self.check_error("viewport");
        }
    }

    #[inline]
    pub unsafe fn draw_elements(
        &self,
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const GLvoid,
    ) {
        unsafe {
            (self.draw_elements)(
                mode,
                count,
                type_,
                indices,
            );
            #[cfg(debug_assertions)]
            self.check_error("draw_elements");
        }
    }

    #[inline]
    pub unsafe fn tex_parameteri(
        &self,
        target: GLenum,
        pname: GLenum,
        param: GLint,
    ) {
        unsafe {
            (self.tex_parameteri)(
                target,
                pname,
                param,
            );
            #[cfg(debug_assertions)]
            self.check_error("tex_parameteri");
        }
    }

    #[inline]
    pub unsafe fn tex_image_2d(
        &self,
        target: GLenum,
        level: GLint,
        internal_format: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const GLvoid,
    ) {
        unsafe {
            (self.tex_image_2d)(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                type_,
                pixels,
            );
            #[cfg(debug_assertions)]
            self.check_error("tex_image_2d");
        }
    }

    #[inline]
    pub unsafe fn gen_textures(
        &self,
        textures: &mut [GLuint],
    ) {
        unsafe {
            (self.gen_textures)(
                textures.len() as GLsizei,
                textures.as_mut_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("gen_textures");
        }
    }

    #[inline]
    pub unsafe fn delete_textures(
        &self,
        textures: &[GLuint],
    ) {
        unsafe {
            (self.delete_textures)(
                textures.len() as GLsizei,
                textures.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("delete_textures");
        }
    }

    #[inline]
    pub unsafe fn bind_texture(
        &self,
        target: GLenum,
        texture: GLuint,
    ) {
        unsafe {
            (self.bind_texture)(
                target,
                texture,
            );
            #[cfg(debug_assertions)]
            self.check_error("bind_texture");
        }
    }

    #[inline]
    pub unsafe fn bind_buffer(
        &self,
        target: GLenum,
        buffer: GLuint,
    ) {
        unsafe {
            (self.bind_buffer)(
                target,
                buffer,
            );
            #[cfg(debug_assertions)]
            self.check_error("bind_buffer");
        }
    }

    #[inline]
    pub unsafe fn delete_buffers(
        &self,
        buffers: &[GLuint],
    ) {
        unsafe {
            (self.delete_buffers)(
                buffers.len() as GLsizei,
                buffers.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("delete_buffers");
        }
    }

    #[inline]
    pub unsafe fn gen_buffers(
        &self,
        buffers: &mut [GLuint],
    ) {
        unsafe {
            (self.gen_buffers)(
                buffers.len() as GLsizei,
                buffers.as_mut_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("gen_buffers");
        }
    }

    #[inline]
    pub unsafe fn buffer_data(
        &self,
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
    ) {
        unsafe {
            (self.buffer_data)(
                target,
                size,
                data,
                usage,
            );
            #[cfg(debug_assertions)]
            self.check_error("buffer_data");
        }
    }

    #[inline]
    pub unsafe fn buffer_sub_data(
        &self,
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void,
    ) {
        unsafe {
            (self.buffer_sub_data)(
                target,
                offset,
                size,
                data,
            );
            #[cfg(debug_assertions)]
            self.check_error("buffer_sub_data");
        }
    }

    #[inline]
    pub unsafe fn map_buffer(
        &self,
        target: GLenum,
        access: GLenum,
    ) -> *mut c_void {
        unsafe {
            let ret = (self.map_buffer)(
                target,
                access,
            );
            #[cfg(debug_assertions)]
            self.check_error("map_buffer");
            ret
        }
    }

    #[inline]
    pub unsafe fn unmap_buffer(
        &self,
        target: GLenum,
    ) -> bool {
        unsafe {
            let ret = (self.unmap_buffer)(
                target,
            ) != 0;
            #[cfg(debug_assertions)]
            self.check_error("unmap_buffer");
            ret
        }
    }

    #[inline]
    pub unsafe fn attach_shader(
        &self,
        program: GLuint,
        shader: GLuint,
    ) {
        unsafe {
            (self.attach_shader)(
                program,
                shader,
            );
            #[cfg(debug_assertions)]
            self.check_error("attach_shader");
        }
    }
    
    #[inline]
    pub unsafe fn compile_shader(
        &self,
        shader: GLuint,
    ) {
        unsafe {
            (self.compile_shader)(
                shader,
            );
            #[cfg(debug_assertions)]
            self.check_error("compile_shader");
        }
    }

    #[inline]
    pub unsafe fn create_program(
        &self,
    ) -> GLuint {
        unsafe {
            let ret = (self.create_program)();
            #[cfg(debug_assertions)]
            self.check_error("create_program");
            ret
        }
    }

    #[inline]
    pub unsafe fn create_shader(
        &self,
        type_: GLenum,
    ) -> GLuint {
        unsafe {
            let ret = (self.create_shader)(
                type_,
            );
            #[cfg(debug_assertions)]
            self.check_error("create_shader");
            ret
        }
    }

    #[inline]
    pub unsafe fn delete_program(
        &self,
        program: GLuint,
    ) {
        unsafe {
            (self.delete_program)(
                program,
            );
            #[cfg(debug_assertions)]
            self.check_error("delete_program");
        }
    }

    #[inline]
    pub unsafe fn delete_shader(
        &self,
        shader: GLuint,
    ) {
        unsafe {
            (self.delete_shader)(
                shader,
            );
            #[cfg(debug_assertions)]
            self.check_error("delete_shader");
        }
    }

    #[inline]
    pub unsafe fn disable_vertex_attrib_array(
        &self,
        index: GLuint,
    ) {
        unsafe {
            (self.disable_vertex_attrib_array)(
                index,
            );
            #[cfg(debug_assertions)]
            self.check_error("disable_vertex_attrib_array");
        }
    }

    #[inline]
    pub unsafe fn enable_vertex_attrib_array(
        &self,
        index: GLuint,
    ) {
        unsafe {
            (self.enable_vertex_attrib_array)(
                index,
            );
            #[cfg(debug_assertions)]
            self.check_error("enable_vertex_attrib_array");
        }
    }

    #[inline]
    pub unsafe fn get_attrib_location(
        &self,
        program: GLuint,
        name: &CStr,
    ) -> GLint {
        unsafe {
            let ret = (self.get_attrib_location)(
                program,
                name.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("get_attrib_location");
            ret
        }
    }

    #[inline]
    pub unsafe fn get_uniform_location(
        &self,
        program: GLuint,
        name: &CStr,
    ) -> GLint {
        unsafe {
            let ret = (self.get_uniform_location)(
                program,
                name.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("get_uniform_location");
            ret
        }
    }

    #[inline]
    pub unsafe fn link_program(
        &self,
        program: GLuint,
    ) {
        unsafe {
            (self.link_program)(
                program,
            );
            #[cfg(debug_assertions)]
            self.check_error("link_program");
        }
    }

    #[inline]
    pub unsafe fn shader_source(
        &self,
        shader: GLuint,
        string: &str,
    ) {
        unsafe {
            let length = [string.len() as GLint];
            let string = [string.as_ptr() as *const GLchar];
            (self.shader_source)(
                shader,
                string.len() as i32,
                string.as_ptr(),
                length.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("shader_source");
        }
    }

    #[inline]
    pub unsafe fn use_program(
        &self,
        program: GLuint
    ) {
        unsafe {
            (self.use_program)(
                program,
            );
            #[cfg(debug_assertions)]
            self.check_error("use_program");
        }
    }

    #[inline]
    pub unsafe fn uniform_matrix4fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    ) {
        unsafe {
            (self.uniform_matrix4fv)(
                location,
                count,
                transpose,
                value,
            );
            #[cfg(debug_assertions)]
            self.check_error("uniform_matrix4fv");
        }
    }

    #[inline]
    pub unsafe fn vertex_attrib_pointer(
        &self,
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) {
        unsafe {
            (self.vertex_attrib_pointer)(
                index,
                size,
                type_,
                normalized,
                stride,
                pointer,
            );
            #[cfg(debug_assertions)]
            self.check_error("vertex_attrib_pointer");
        }
    }

    #[inline]
    pub unsafe fn bind_framebuffer(
        &self,
        target: GLenum,
        framebuffer: GLuint,
    ) {
        unsafe {
            (self.bind_framebuffer)(
                target,
                framebuffer,
            );
            #[cfg(debug_assertions)]
            self.check_error("bind_framebuffer");
        }
    }

    #[inline]
    pub unsafe fn bind_vertex_array(
        &self,
        array: GLuint,
    ) {
        unsafe {
            (self.bind_vertex_array)(
                array,
            );
            #[cfg(debug_assertions)]
            self.check_error("bind_vertex_array");
        }
    }

    #[inline]
    pub unsafe fn delete_vertex_arrays(
        &self,
        arrays: &[GLuint],
    ) {
        unsafe {
            (self.delete_vertex_arrays)(
                arrays.len() as GLsizei,
                arrays.as_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("delete_vertex_arrays");
        }
    }

    #[inline]
    pub unsafe fn gen_vertex_arrays(
        &self,
        arrays: &mut [GLuint],
    ) {
        unsafe {
            (self.gen_vertex_arrays)(
                arrays.len() as GLsizei,
                arrays.as_mut_ptr(),
            );
            #[cfg(debug_assertions)]
            self.check_error("gen_vertex_arrays");
        }
    }

    #[inline]
    pub unsafe fn copy_buffer_sub_data(
        &self,
        read_target: GLenum,
        write_target: GLenum,
        read_offset: GLintptr,
        write_offset: GLintptr,
        size: GLsizeiptr,
    ) {
        unsafe {
            (self.copy_buffer_sub_data)(
                read_target,
                write_target,
                read_offset,
                write_offset,
                size,
            );
            #[cfg(debug_assertions)]
            self.check_error("copy_buffer_sub_data");
        }
    }
}
