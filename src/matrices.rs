pub struct MatrixOperation {}

impl MatrixOperation {
    pub fn perspective(
        display_ratio: f32,
        frustum_scale: f32,
        z_near: f32,
        z_far: f32,
    ) -> [[f32; 4]; 4] {
        [
            [frustum_scale / display_ratio, 0.0, 0.0, 0.0],
            [0.0, frustum_scale, 0.0, 0.0],
            [
                0.0,
                0.0,
                (z_far + z_near) / (z_near - z_far),
                (2.0 * z_far * z_near) / (z_near - z_far),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ]
    }
}
