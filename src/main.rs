#[macro_use]
extern crate glium;

mod matrices;
mod primitives;
mod shaders;

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
        &VertexShader::color_model_camera_clip(),
        &FragmentShader::smooth_color(),
        None,
    )
    .unwrap();

    let mut perspective_matrix = MatrixOperation::perspective(1.0, 1.0, 1.0, 3.0);

    let draw_parameters = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

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
                    perspective_matrix = MatrixOperation::perspective(ratio, 1.0, 1.0, 30.0);
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

        let uniforms = uniform! {
            modelToCameraMatrix: MatrixOperation::translation(1.5, 1.5, -3.0),
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
