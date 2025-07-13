mod detail {
    mod type_mat4x4;
    mod type_vec1;
    mod type_vec2;
    mod type_vec3;
    mod type_vec4;

    pub use type_mat4x4::*;
    pub use type_vec1::*;
    pub use type_vec2::*;
    pub use type_vec3::*;
    pub use type_vec4::*;
}
mod ext {
    mod matrix_clip_space;
    mod matrix_transform;

    pub use matrix_clip_space::*;
    pub use matrix_transform::*;
}

pub use detail::*;
pub use ext::*;
