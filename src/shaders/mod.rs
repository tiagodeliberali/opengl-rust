use std::fs::read_to_string;

pub struct VertexShader {}

impl VertexShader {
    pub fn color_camera_clip() -> String {
        read_to_string("src/shaders/color_camera_clip.vert").unwrap()
    }
}

pub struct FragmentShader {}

impl FragmentShader {
    pub fn smooth_color() -> String {
        read_to_string("src/shaders/smooth_color.frag").unwrap()
    }
}
