use crate::math::{Matrix4, Vector3};

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

    pub fn translation(input: Vector3) -> Matrix4 {
        Matrix4::from([
            1.0, 0.0, 0.0, input.x,
            0.0, 1.0, 0.0, input.y,
            0.0, 0.0, 1.0, input.z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(input: Vector3) -> Matrix4 {
        Matrix4::from([
            input.x,    0.0,        0.0,        0.0,
            0.0,        input.y,    0.0,        0.0,
            0.0,        0.0,        input.z,    0.0,
            0.0,        0.0,        0.0,        1.0,
        ])
    }

    pub fn camera_matrix(camera_position: Vector3, target: Vector3, up_vector: Vector3) -> Matrix4 {
        let look_direction = (target - camera_position).normalized();
        let up_vector = up_vector.normalized();

        let right_direction = look_direction.cross(up_vector);
        let perpedicular_up_direction = right_direction.cross(look_direction);

        let rotation_matrix = Matrix4::from([
            right_direction.x, right_direction.y, right_direction.z, 0.0,
            perpedicular_up_direction.x, perpedicular_up_direction.y, perpedicular_up_direction.z, 0.0,
            look_direction.x, look_direction.y, look_direction.z, 0.0,
            0.0, 0.0, 0.0, 1.0
        ]);

        let translation_matrix = Matrix4::from([
            1.0, 0.0, 0.0, -camera_position.x,
            0.0, 1.0, 0.0, -camera_position.y,
            0.0, 0.0, 1.0, -camera_position.z,
            0.0, 0.0, 0.0, 1.0
        ]);

        return rotation_matrix * translation_matrix;
    }
}
