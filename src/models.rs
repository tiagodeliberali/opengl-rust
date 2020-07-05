use crate::math::{Matrix4, Quaternion, Vector3};
use crate::matrices::MatrixOperation;
use crate::primitives::Vertex;
use crate::shaders::{FragmentShader, VertexShader};

use glium::backend::glutin::Display;
use glium::{DrawParameters, Program};
use glium::{IndexBuffer, VertexBuffer};
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use glium::glutin::{
    event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode},
    event_loop::EventLoop,
};
use glium::{glutin, Surface};

const Z_NEAR: f32 = 1.0;
const Z_FAR: f32 = 1000.0;
const VIEW_ANGLE: f32 = 45.0;

pub struct World<'a> {
    pub display: Display,
    draw_parameters: DrawParameters<'a>,
    program: Program,
    perspective_matrix: Matrix4,
    camera: Camera,
    device_manager: DeviceManager,
    instances: HashMap<String, Instance>,
    update: Option<Box<dyn FnMut(&DeviceManager, &mut HashMap<String, Instance>, &mut Camera)>>,
}

impl<'a> World<'static> {
    pub fn new(event_loop: &EventLoop<()>, camera: Camera) -> World<'static> {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Hello OpenGL - focus on game math")
            .with_inner_size(glutin::dpi::LogicalSize::new(600.0, 600.0));

        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        // responsible for handling shaders
        // uniforms are completely related to shaders, so it must be improved to be a generic solution
        let program = glium::Program::from_source(
            &display,
            &VertexShader::color_world_model_camera_clip(),
            &FragmentShader::smooth_color(),
            None,
        )
        .unwrap();

        let perspective_matrix = MatrixOperation::perspective(1.0, VIEW_ANGLE, Z_NEAR, Z_FAR);

        let device_manager = DeviceManager::new();

        World {
            display,
            draw_parameters,
            program,
            perspective_matrix,
            camera,
            device_manager,
            instances: HashMap::new(),
            update: None,
        }
    }

    pub fn add_instance(&mut self, name: String, instance: Instance) {
        self.instances.insert(name, instance);
    }

    pub fn update_key_manager(&mut self, input: &KeyboardInput) {
        self.device_manager.update_keys(input);
    }

    pub fn update_mouse_motion(&mut self, delta: (f64, f64)) {
        self.device_manager.update_mouse_motion(delta.0, delta.1);
    }

    pub fn update_mouse_wheel(&mut self, delta: MouseScrollDelta) {}

    pub fn draw_update(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        if let Some(update_action) = &mut self.update {
            update_action(&self.device_manager, &mut self.instances, &mut self.camera);

            for instance in self.instances.values() {
                let uniforms = uniform! {
                    modelToWorldMatrix: instance.operations,
                    worldToCameraMatrix: self.camera.camera_matrix_from_target(),
                    cameraToClipMatrix: self.perspective_matrix
                };

                target
                    .draw(
                        &instance.prefab.vertex,
                        &instance.prefab.indices,
                        &self.program,
                        &uniforms,
                        &self.draw_parameters,
                    )
                    .unwrap();
            }
        }

        target.finish().unwrap();
        self.device_manager.reset_mouse();
    }

    pub fn set_update<F>(&mut self, update_fn: F)
    where
        F: 'static + FnMut(&DeviceManager, &mut HashMap<String, Instance>, &mut Camera),
    {
        self.update.replace(Box::from(update_fn));
    }

    pub fn change_perspective_ratio(&mut self, ratio: f32) {
        self.perspective_matrix = MatrixOperation::perspective(ratio, VIEW_ANGLE, Z_NEAR, Z_FAR);
    }
}

pub struct Prefab {
    vertex: VertexBuffer<Vertex>,
    indices: IndexBuffer<u16>,
}

impl Prefab {
    pub fn build(display: Display, shape: Vec<Vertex>, indices: Vec<u16>) -> Arc<Self> {
        let vertex = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(
            &display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        Arc::new(Prefab { vertex, indices })
    }
}

pub struct Camera {
    pub operations: Matrix4,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            operations: Matrix4::identity(),
        }
    }

    pub fn set_parent(&mut self, instance: &Instance) {
        self.operations = instance.operations.clone();
    }

    /// original version: MatrixOperation::camera_matrix(self.camera_position, self.target_position, Vector3::up())
    #[rustfmt::skip]
    pub fn camera_matrix_from_target(&self) -> Matrix4 {
        let look_direction = self.operations.get_forward_vector().normalized();
        let up_vector = self.operations.get_up_vector().normalized();

        let right_direction = look_direction.cross(up_vector);
        let perpedicular_up_direction = right_direction.cross(look_direction);

        let rotation_matrix = Matrix4::from([
            right_direction.x,              right_direction.y,              right_direction.z,              0.0,
            perpedicular_up_direction.x,    perpedicular_up_direction.y,    perpedicular_up_direction.z,    0.0,
            -look_direction.x,              -look_direction.y,              -look_direction.z,              0.0,
            0.0,                            0.0,                            0.0,                            1.0,
        ]);

        let camera_position = self.operations.get_position();

        let translation_matrix = Matrix4::from([
            1.0,    0.0,    0.0,    -camera_position.x,
            0.0,    1.0,    0.0,    -camera_position.y,
            0.0,    0.0,    1.0,    -camera_position.z,
            0.0,    0.0,    0.0,    1.0,
        ]);

        return rotation_matrix * translation_matrix;
    }
}

pub struct Instance {
    operations: Matrix4,
    prefab: Arc<Prefab>,
}

#[allow(dead_code)]
impl Instance {
    pub fn new(prefab: Arc<Prefab>) -> Self {
        Instance {
            operations: Matrix4::identity(),
            prefab,
        }
    }

    pub fn clone(&self) -> Self {
        Instance {
            operations: self.operations.clone(),
            prefab: self.prefab.clone(),
        }
    }

    pub fn set_parent(&mut self, parent: &Instance) {
        self.operations =
            MatrixOperation::translation(parent.operations.get_position()) * self.operations;
    }

    pub fn reset_transform(&mut self) {
        self.operations = Matrix4::identity();
    }

    pub fn set_scale(&mut self, vector: Vector3) {
        self.operations = MatrixOperation::scale(vector) * self.operations;
    }

    pub fn set_translation(&mut self, vector: Vector3) {
        self.operations = MatrixOperation::translation(vector) * self.operations;
    }

    pub fn set_rotate_x(&mut self, angle: f32) {
        self.operations = self.operations * Quaternion::rotate_x(angle);
    }

    pub fn set_rotate_y(&mut self, angle: f32) {
        self.operations = self.operations * Quaternion::rotate_y(angle);
    }

    pub fn set_rotate_z(&mut self, angle: f32) {
        self.operations = self.operations * Quaternion::rotate_z(angle);
    }

    pub fn add_front_translation(&mut self, amount: f32) {
        let vector = self.operations.get_forward_vector().normalized();
        let vector = vector * amount;
        self.operations = MatrixOperation::translation(vector) * self.operations;
    }

    pub fn add_side_translation(&mut self, amount: f32) {
        let vector = self.operations.get_side_vector().normalized();
        let vector = vector * amount;
        self.operations = MatrixOperation::translation(vector) * self.operations;
    }

    pub fn add_up_translation(&mut self, amount: f32) {
        let vector = self.operations.get_up_vector().normalized();
        let vector = vector * amount;
        self.operations = MatrixOperation::translation(vector) * self.operations;
    }
}

pub struct DeviceManager {
    pressed_keys: HashSet<VirtualKeyCode>,
    mouse_delta_y: Option<f32>,
    mouse_delta_x: Option<f32>,
}

const MIN_CHANGE: f64 = 0.001;

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            pressed_keys: HashSet::new(),
            mouse_delta_y: None,
            mouse_delta_x: None,
        }
    }

    pub fn update_keys(&mut self, input: &KeyboardInput) {
        if let Some(code) = input.virtual_keycode {
            match input.state {
                ElementState::Pressed => self.pressed_keys.insert(code),
                ElementState::Released => self.pressed_keys.remove(&code),
            };
        }
    }

    pub fn update_mouse_motion(&mut self, x: f64, y: f64) {
        self.mouse_delta_x = if x.abs() > MIN_CHANGE {
            Some(x as f32)
        } else {
            None
        };

        self.mouse_delta_y = if y.abs() > MIN_CHANGE {
            Some(y as f32)
        } else {
            None
        };
    }

    pub fn iter_keys(&self) -> Iter<VirtualKeyCode> {
        self.pressed_keys.iter()
    }

    pub fn get_last_mouse_movement_x(&self) -> f32 {
        if let Some(value) = self.mouse_delta_x {
            value
        } else {
            0.0
        }
    }

    pub fn get_last_mouse_movement_y(&self) -> f32 {
        if let Some(value) = self.mouse_delta_y {
            value
        } else {
            0.0
        }
    }

    pub fn reset_mouse(&mut self) {
        self.mouse_delta_x = None;
        self.mouse_delta_y = None;
    }
}
