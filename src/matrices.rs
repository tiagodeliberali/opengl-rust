use glium::uniforms::{AsUniformValue, UniformValue};
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4 {
    data: [f32; 16],
}

impl AsUniformValue for Matrix4 {
    #[inline]
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Mat4(self.to_opengl_array())
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    #[rustfmt::skip]
    fn mul(self, other: Matrix4) -> Self::Output {
        Matrix4::from([
            self.data[0] * other.data[0] + self.data[1] * other.data[4] + self.data[2] * other.data[8] + self.data[3] * other.data[12],
            self.data[0] * other.data[1] + self.data[1] * other.data[5] + self.data[2] * other.data[9] + self.data[3] * other.data[13],
            self.data[0] * other.data[2] + self.data[1] * other.data[6] + self.data[2] * other.data[10] + self.data[3] * other.data[14],
            self.data[0] * other.data[3] + self.data[1] * other.data[7] + self.data[2] * other.data[11] + self.data[3] * other.data[15],

            self.data[4] * other.data[0] + self.data[5] * other.data[4] + self.data[6] * other.data[8] + self.data[7] * other.data[12],
            self.data[4] * other.data[1] + self.data[5] * other.data[5] + self.data[6] * other.data[9] + self.data[7] * other.data[13],
            self.data[4] * other.data[2] + self.data[5] * other.data[6] + self.data[6] * other.data[10] + self.data[7] * other.data[14],
            self.data[4] * other.data[3] + self.data[5] * other.data[7] + self.data[6] * other.data[11] + self.data[7] * other.data[15],

            self.data[8] * other.data[0] + self.data[9] * other.data[4] + self.data[10] * other.data[8] + self.data[11] * other.data[12],
            self.data[8] * other.data[1] + self.data[9] * other.data[5] + self.data[10] * other.data[9] + self.data[11] * other.data[13],
            self.data[8] * other.data[2] + self.data[9] * other.data[6] + self.data[10] * other.data[10] + self.data[11] * other.data[14],
            self.data[8] * other.data[3] + self.data[9] * other.data[7] + self.data[10] * other.data[11] + self.data[11] * other.data[15],

            self.data[12] * other.data[0] + self.data[13] * other.data[4] + self.data[14] * other.data[8] + self.data[15] * other.data[12],
            self.data[12] * other.data[1] + self.data[13] * other.data[5] + self.data[14] * other.data[9] + self.data[15] * other.data[13],
            self.data[12] * other.data[2] + self.data[13] * other.data[6] + self.data[14] * other.data[10] + self.data[15] * other.data[14],
            self.data[12] * other.data[3] + self.data[13] * other.data[7] + self.data[14] * other.data[11] + self.data[15] * other.data[15],
        ])
    }
}

impl Matrix4 {
    pub fn from(data: [f32; 16]) -> Matrix4 {
        Matrix4 { data }
    }

    #[rustfmt::skip]
    pub fn to_opengl_array(self) -> [[f32; 4]; 4] {
        [
            [self.data[0], self.data[4], self.data[8], self.data[12]],
            [self.data[1], self.data[5], self.data[9], self.data[13]],
            [self.data[2], self.data[6], self.data[10], self.data[14]],
            [self.data[3], self.data[7], self.data[11], self.data[15]],
        ]
    }
}
pub struct MatrixOperation {}

#[rustfmt::skip]
impl MatrixOperation {
    pub fn perspective(
        display_ratio: f32,
        frustum_scale: f32,
        z_near: f32,
        z_far: f32,
    ) -> Matrix4 {
        Matrix4::from([
            frustum_scale / display_ratio, 0.0, 0.0, 0.0,
            0.0, frustum_scale, 0.0, 0.0,
            0.0, 0.0, (z_far + z_near) / (z_near - z_far), (2.0 * z_far * z_near) / (z_near - z_far),
            0.0, 0.0, -1.0, 0.0
        ])
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::from([
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4::from([
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
}
