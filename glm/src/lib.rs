mod detail {
    mod type_vec1;
    mod type_vec2;

    pub use type_vec1::*;
    pub use type_vec2::*;
}

pub use detail::*;
