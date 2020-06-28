use std::fs::read_to_string;

pub struct VertexShader {}

#[allow(dead_code)]
impl VertexShader {
    pub fn color_model_camera_clip() -> String {
        read_to_string("src/shaders/color_model_camera_clip.vert").unwrap()
    }

    pub fn color_world_model_camera_clip() -> String {
        read_to_string("src/shaders/color_world_model_camera_clip.vert").unwrap()
    }
}

pub struct FragmentShader {}

impl FragmentShader {
    pub fn smooth_color() -> String {
        read_to_string("src/shaders/smooth_color.frag").unwrap()
    }
}
