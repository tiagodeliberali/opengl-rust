use crate::math::{clamp, degree_to_radians, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct SphereVector {
    phi: f32,
    theta: f32,
    radius: f32,
}

impl SphereVector {
    pub fn new(phi: f32, theta: f32, radius: f32) -> SphereVector {
        SphereVector {
            phi,
            theta: clamp(theta, -78.75, 78.75),
            radius: f32::max(1.0, radius),
        }
    }

    pub fn to_euclidean(self, target: Vector3) -> Vector3 {
        let phi = degree_to_radians(self.phi);
        let theta = degree_to_radians(self.theta + 90.0);

        let sin_theta = f32::sin(theta);
        let cos_theta = f32::cos(theta);
        let cos_phi = f32::cos(phi);
        let sin_phi = f32::sin(phi);

        let camera_direction = Vector3::new(sin_theta * cos_phi, cos_theta, sin_theta * sin_phi);
        (camera_direction * self.radius) + target
    }
}
