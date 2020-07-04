use crate::math::{Matrix4, Quaternion, Vector3};
use crate::matrices::MatrixOperation;
use crate::primitives::Vertex;
use crate::shaders::{FragmentShader, VertexShader};

use glium::backend::glutin::Display;
use glium::{DrawParameters, Program};
use glium::{IndexBuffer, VertexBuffer};
use std::sync::Arc;

use glium::glutin::event_loop::EventLoop;
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
    update: Option<Box<dyn FnMut() -> Vec<Instance>>>,
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

        World {
            display,
            draw_parameters,
            program,
            perspective_matrix,
            camera,
            update: None,
        }
    }

    pub fn update_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn draw_update(&mut self) {
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        if let Some(update_action) = &mut self.update {
            let instances = update_action();

            for instance in instances {
                let uniforms = uniform! {
                    modelToWorldMatrix: instance.operations,
                    worldToCameraMatrix: self.camera.build_camera_matrix(),
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
    }

    pub fn set_update<F>(&mut self, update_fn: F)
    where
        F: 'static + FnMut() -> Vec<Instance>,
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

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub camera_position: Vector3,
    pub target_position: Vector3,
}

impl Camera {
    pub fn new(camera_position: Vector3, target_position: Vector3) -> Camera {
        Camera {
            camera_position,
            target_position,
        }
    }

    pub fn build_camera_matrix(&self) -> Matrix4 {
        MatrixOperation::camera_matrix(self.camera_position, self.target_position, Vector3::up())
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

    pub fn set_scale(&mut self, vector: Vector3) {
        self.operations = MatrixOperation::scale(vector) * self.operations;
    }

    pub fn set_translation(&mut self, vector: Vector3) {
        self.operations = MatrixOperation::translation(vector) * self.operations;
    }

    pub fn set_rotate_x(&mut self, angle: f32) {
        self.operations = Quaternion::rotate_x(angle) * self.operations;
    }

    pub fn set_rotate_y(&mut self, angle: f32) {
        self.operations = Quaternion::rotate_y(angle) * self.operations;
    }

    pub fn set_rotate_z(&mut self, angle: f32) {
        self.operations = Quaternion::rotate_z(angle) * self.operations;
    }
}
