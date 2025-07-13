use crate::mat4x4;

impl mat4x4<f32> {
    #[inline]
    pub fn ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
    ) -> Self {
        let mut result = Self::default();
        result[0][0] = 2 as f32 / (right - left);
        result[1][1] = 2 as f32 / (top - bottom);
        result[2][2] = - 1 as f32;
        result[3][0] = - (right + left) / (right - left);
        result[3][1] = - (top + bottom) / (top - bottom);
        result
    }
}
