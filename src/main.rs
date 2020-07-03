#[macro_use]
extern crate glium;

mod coordinates;
mod math;
mod matrices;
mod models;
mod primitives;
mod shaders;

use coordinates::SphereVector;
use glium::glutin;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use math::Vector3;
use models::{Instance, World};
use primitives::Primitive;

fn main() {
    let event_loop = EventLoop::new();
    let mut world = World::new(&event_loop);

    // ITEMS TO DRAW
    let cube_prefab = Primitive::cube(world.display.clone());

    let mut step = 0;

    world.set_update(move || {
        let mut instances = Vec::new();

        // MOVING BLOCK ON SPHERICAL COORDINATES
        step = step + 1;
        step = step % 360;

        let mut cube_instance1 = Instance::new(cube_prefab.clone());
        cube_instance1.set_rotate_z(step as f32);
        cube_instance1.set_scale(Vector3::new(1.5, 1.5, 1.5));
        cube_instance1.set_translation(Vector3::new(0.0, 0.0, -5.0));

        instances.push(cube_instance1);

        let mut cube_instance2 = Instance::new(cube_prefab.clone());
        cube_instance2.set_scale(Vector3::new(0.5, 0.5, 0.5));
        cube_instance2.set_translation(Vector3::new(0.0, 0.0, -5.0));
        cube_instance2.set_translation(SphereVector::new(1.5, -20.0, step as f32).to_cartesian());

        instances.push(cube_instance2);

        let mut cube_instance3 = Instance::new(cube_prefab.clone());
        cube_instance3.set_scale(Vector3::new(0.5, 0.5, 0.5));
        cube_instance3.set_translation(Vector3::new(0.0, 0.0, -5.0));
        cube_instance3
            .set_translation(SphereVector::new(2.0, -45.0, step as f32 + 90.0).to_cartesian());

        instances.push(cube_instance3);

        instances
    });

    let mut next_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if next_frame_time.elapsed() > std::time::Duration::from_nanos(16_666_667) {
            world.draw_update();
            next_frame_time = std::time::Instant::now();
        }

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
                _ => (),
            },
            _ => (),
        }
    });
}
