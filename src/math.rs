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

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn up() -> Self {
        Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn normalized(self) -> Self {
        let norm: f32 = (self.x.exp2() + self.y.exp2() + self.z.exp2()).sqrt();
        let mut result = self.clone();

        result.x /= norm;
        result.y /= norm;
        result.z /= norm;

        result
    }

    pub fn cross(self, other: Vector3) -> Self {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        Vector3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl AsUniformValue for Vector3 {
    #[inline]
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec3(self.to_array())
    }
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    f32::min(f32::max(value, min), max)
}

const DEGREE_TO_RADIANS_RATION: f32 = std::f32::consts::PI * 2.0 / 360.0;
pub fn degree_to_radians(angle: f32) -> f32 {
    angle * DEGREE_TO_RADIANS_RATION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_normalized() {
        let a = Vector3::new(3.0, 1.0, 2.0);

        assert_eq!(a.normalized(), Vector3::new(0.8017837, 0.26726124, 0.5345225));
    }
    
    #[test]
    fn vector3_to_array() {
        let a = Vector3::new(3.0, 1.0, 2.0);

        assert_eq!(a.to_array(), [3.0, 1.0, 2.0]);
    }

    #[test]
    fn vector3_cross() {
        let a = Vector3::new(2.0, 3.0, 4.0);
        let b = Vector3::new(5.0, 6.0, 7.0);

        assert_eq!(a.cross(b), Vector3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn vector3_cross_zeroed() {
        let a = Vector3::new(3.0, -3.0, 1.0);
        let b = Vector3::new(12.0, -12.0, 4.0);

        assert_eq!(a.cross(b), Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn clamp_values() {
        assert_eq!(5.0, clamp(3.0, 5.0, 10.0));

        assert_eq!(10.0, clamp(18.0, 5.0, 10.0));

        assert_eq!(7.0, clamp(7.0, 5.0, 10.0));
    }

    #[test]
    fn degree_to_radians_values() {
        assert_eq!(0.017453292, degree_to_radians(1.0));
        
        assert_eq!(0.5235988, degree_to_radians(30.0));

        assert_eq!(0.0, degree_to_radians(0.0));

        assert_eq!(-0.7853982, degree_to_radians(-45.0));
    }
}
