#[macro_use]
extern crate glium;

mod coordinates;
mod math;
mod matrices;
mod models;
mod primitives;
mod shaders;

use coordinates::SphereVector;
use math::Vector3;
use models::{Instance, World};
use primitives::Primitive;

use glium::backend::glutin::Display;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::{glutin, Surface};

fn config_display(event_loop: &EventLoop<()>) -> Display {
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Hello OpenGL - focus on game math")
        .with_inner_size(glutin::dpi::LogicalSize::new(600.0, 600.0));

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    display
}

fn main() {
    let event_loop = EventLoop::new();
    let display = config_display(&event_loop);

    let mut world = World::new(display.clone());

    // ITEMS TO DRAW
    let cube_prefab = Primitive::cube(display.clone());

    let mut cube_instance1 = Instance::new(cube_prefab.clone());
    cube_instance1.set_scale(Vector3::new(1.5, 1.5, 1.5));
    cube_instance1.set_translation(Vector3::new(0.0, 0.0, -5.0));

    let mut step = 0;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(new_size) => {
                    let ratio = new_size.width as f32 / new_size.height as f32;
                    world.change_perspective_ratio(ratio);
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

        // MOVING BLOCK ON SPHERICAL COORDINATES
        step += 1;
        step = step % 360;

        let mut cube_instance2 = Instance::new(cube_prefab.clone());
        cube_instance2.set_scale(Vector3::new(0.5, 0.5, 0.5));
        cube_instance2.set_translation(Vector3::new(0.0, 0.0, -5.0));
        cube_instance2.set_translation(SphereVector::new(1.5, -20.0, step as f32).to_cartesian());

        world.draw(&mut target, &cube_instance2);

        let mut cube_instance3 = Instance::new(cube_prefab.clone());
        cube_instance3.set_scale(Vector3::new(0.5, 0.5, 0.5));
        cube_instance3.set_translation(Vector3::new(0.0, 0.0, -5.0));
        cube_instance3
            .set_translation(SphereVector::new(2.0, -45.0, step as f32 + 90.0).to_cartesian());

        world.draw(&mut target, &cube_instance3);

        // TARGET CAMERA BLOCK
        world.draw(&mut target, &cube_instance1);

        target.finish().unwrap();
    });
}
