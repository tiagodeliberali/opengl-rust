use crate::math::{clamp, degree_to_radians, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct SphereVector {
    radius: f32,
    azimuthal: f32,
    elevation: f32,
}

/// reference: https://en.wikipedia.org/wiki/Spherical_coordinate_system
/// remember that up is y, not z, so the elevation angle is against y, not z.
/// clamp is necessary to avoid problems on matrix camera transform. Reference: https://paroj.github.io/gltut/Positioning/Tutorial%2007.html
impl SphereVector {
    pub fn new(radius: f32, elevation: f32, azimuthal: f32) -> SphereVector {
        SphereVector {
            radius: f32::max(1.0, radius),
            azimuthal,
            elevation: clamp(elevation, -78.75, 78.75),
        }
    }

    pub fn to_cartesian(self) -> Vector3 {
        let elevation = degree_to_radians(self.elevation + 90.0);
        let azimuthal = degree_to_radians(self.azimuthal);

        let sin_elevation = f32::sin(elevation);
        let cos_elevation = f32::cos(elevation);
        let cos_azimuthal = f32::cos(azimuthal);
        let sin_azimuthal = f32::sin(azimuthal);

        let camera_direction = Vector3::new(
            sin_elevation * cos_azimuthal,
            cos_elevation,
            sin_elevation * sin_azimuthal,
        );
        camera_direction * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_vector_clamp_elevation() {
        let clamped_max = SphereVector::new(100.0, 90.0, 10.0);
        let clamped_min = SphereVector::new(100.0, -80.0, 10.0);
        let not_clampeda = SphereVector::new(100.0, 45.0, 10.0);

        assert_eq!(78.75, clamped_max.elevation);
        assert_eq!(-78.75, clamped_min.elevation);
        assert_eq!(45.0, not_clampeda.elevation);
    }

    #[test]
    fn sphere_vector_clamp_radius() {
        let clamped_min = SphereVector::new(100.0, 45.0, -10.0);
        let not_clampeda = SphereVector::new(100.0, 45.0, 10.0);

        assert_eq!(1.0, clamped_min.radius);
        assert_eq!(10.0, not_clampeda.radius);
    }

    #[test]
    fn sphere_vector_to_cartesian() {
        assert_eq!(
            Vector3::new(1.0, -0.00000004371139, 0.0),
            SphereVector::new(1.0, 0.0, 0.0).to_cartesian()
        );

        assert_eq!(
            Vector3::new(-0.00000004371139, -0.00000004371139, 1.0),
            SphereVector::new(1.0, 0.0, 90.0).to_cartesian()
        );

        assert_eq!(
            Vector3::new(0.8660253, -1.0000001, 1.5),
            SphereVector::new(2.0, 30.0, 60.0).to_cartesian()
        );
    }
}
