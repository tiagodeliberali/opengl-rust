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
use glium::glutin::event::VirtualKeyCode;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use math::Vector3;
use models::{Camera, Instance, World};
use primitives::Primitive;

fn main() {
    let event_loop = EventLoop::new();
    let camera = Camera::new(Vector3::new(-2.0, 3.0, -8.0), Vector3::new(0.0, 0.0, -5.0));
    let mut world = World::new(&event_loop, camera.clone());

    // ITEMS TO DRAW
    let cube_prefab = Primitive::cube(world.display.clone());

    let mut cube_instance = Instance::new(cube_prefab.clone());
    cube_instance.set_scale(Vector3::new(1.5, 1.5, 1.5));
    cube_instance.set_translation(Vector3::new(0.0, 0.0, -5.0));
    world.add_instance(String::from("instance1"), cube_instance);

    world.add_instance(
        String::from("instance2"),
        Instance::new(cube_prefab.clone()),
    );

    world.add_instance(
        String::from("instance3"),
        Instance::new(cube_prefab.clone()),
    );

    // DRAW STEP
    let mut step = 0;

    world.set_update(move |key_manager, instances| {
        let mut front_movement = 0.0;
        let mut side_movement = 0.0;
        let mut up_movement = 0.0;

        for key in key_manager.iter() {
            match key {
                VirtualKeyCode::W => front_movement = 0.1,
                VirtualKeyCode::S => front_movement = -0.1,
                VirtualKeyCode::A => side_movement = 0.1,
                VirtualKeyCode::D => side_movement = -0.1,
                VirtualKeyCode::Q => up_movement = 0.1,
                VirtualKeyCode::E => up_movement = -0.1,
                _ => (),
            };
        }

        // MOVING BLOCK ON SPHERICAL COORDINATES
        step = step + 1;
        step = step % 360;

        instances
            .entry(String::from("instance1"))
            .and_modify(|instance| {
                instance.set_rotate_z(1.0);
                instance.add_front_translation(front_movement);
                instance.add_side_translation(side_movement);
                instance.add_up_translation(up_movement);
            });

        instances
            .entry(String::from("instance2"))
            .and_modify(|instance| {
                instance.reset_transform();
                instance.set_scale(Vector3::new(0.5, 0.5, 0.5));
                instance.set_translation(Vector3::new(0.0, 0.0, -5.0));
                instance.set_translation(SphereVector::new(1.5, -20.0, step as f32).to_cartesian());
            });

        instances
            .entry(String::from("instance3"))
            .and_modify(|instance| {
                instance.reset_transform();
                instance.set_scale(Vector3::new(0.5, 0.5, 0.5));
                instance.set_translation(Vector3::new(0.0, 0.0, -5.0));
                instance.set_translation(
                    SphereVector::new(2.0, -45.0, step as f32 + 90.0).to_cartesian(),
                );
            });
    });

    // WIN EVENT LOOP
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
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    world.update_key_manager(&input)
                }
                _ => (),
            },
            _ => (),
        }
    });
}
