#[macro_use]
extern crate glium;

mod primitives;
mod shaders;

use primitives::Primitive;
use shaders::{FragmentShader, VertexShader};

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
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
        &VertexShader::color_camera_clip(),
        &FragmentShader::smooth_color(),
        None,
    )
    .unwrap();

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


        let frustum_scale: f32 = 1.0;
        let z_near = 1.0_f32;
        let z_far = 3.0_f32;

        let uniforms = uniform! {
            offset: [0.75_f32, 0.75_f32, -1.0_f32],
            perspectiveMatrix: [
                [frustum_scale,     0.0,            0.0,                                    0.0],
                [0.0,               frustum_scale,  0.0,                                    0.0],
                [0.0,               0.0,            (z_far + z_near) / (z_near - z_far),    (2.0 * z_far * z_near) / (z_near - z_far)],
                [0.0,               0.0,            -1.0,                                   0.0],
            ]
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        target
            .draw(&vertex_buffer, &indices, &program, &uniforms, &params)
            .unwrap();
        target.finish().unwrap();
    });
}
