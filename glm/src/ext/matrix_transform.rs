use crate::{mat4x4, vec3};

impl mat4x4<f32> {
    #[inline]
    pub fn translate(
        &self,
        v: vec3<f32>
    ) -> Self {
        let mut result = *self;
        result[3] = self[0] * v[0] + self[1] * v[1] + self[2] * v[2] + self[3];
        result
    }
}
