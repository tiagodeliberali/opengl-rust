#[macro_use]
extern crate glium;

mod coordinates;
mod math;
mod matrices;
mod primitives;
mod shaders;

use coordinates::SphereVector;
use math::Vector3;
use matrices::MatrixOperation;
use primitives::Primitive;
use shaders::{FragmentShader, VertexShader};

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello OpenGL - focus on game math")
        .with_inner_size(glutin::dpi::LogicalSize::new(600.0, 600.0));

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let (shape, indices) = Primitive::cube();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    let program = glium::Program::from_source(
        &display,
        &VertexShader::color_world_model_camera_clip(),
        &FragmentShader::smooth_color(),
        None,
    )
    .unwrap();

    let mut perspective_matrix = MatrixOperation::perspective(1.0, 1.0, 1.0, 1000.0);

    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let mut step = 0;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(new_size) => {
                    let ratio = new_size.width as f32 / new_size.height as f32;
                    perspective_matrix = MatrixOperation::perspective(ratio, 1.0, 1.0, 1000.0);
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let camera_target = Vector3::new(0.0, 0.0, 0.0);
        let camera_position = SphereVector::new(5.0, 30.0, 0.0);
        let camera_position = camera_position.to_cartesian() + camera_target;

        step += 1;
        step = step % 360;

        let instance_position = MatrixOperation::translation(SphereVector::new(1.5, -20.0, step as f32).to_cartesian())
            * MatrixOperation::translation(Vector3::new(0.0, 0.0, -5.0))
            * MatrixOperation::scale(Vector3::new(0.5, 0.5, 0.5));

        let uniforms = uniform! {
            modelToWorldMatrix: instance_position,
            worldToCameraMatrix: MatrixOperation::camera_matrix(camera_position, camera_target, Vector3::up()),
            cameraToClipMatrix: perspective_matrix
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();

        let uniforms = uniform! {
            modelToWorldMatrix: MatrixOperation::translation(Vector3::new(0.0, 0.0, -5.0)),
            worldToCameraMatrix: MatrixOperation::camera_matrix(camera_position, camera_target, Vector3::up()),
            cameraToClipMatrix: perspective_matrix
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();

        target.finish().unwrap();
    });
}
