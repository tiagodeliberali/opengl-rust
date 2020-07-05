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
use math::{clamp, Vector3};
use models::{Camera, Instance, World};
use primitives::Primitive;

const MOUSE_SENSIBILITY: f32 = 0.5;

fn main() {
    let event_loop = EventLoop::new();
    let camera = Camera::new();
    let mut world = World::new(&event_loop, camera);

    // ITEMS TO DRAW
    let cube_prefab = Primitive::cube(world.display.clone());

    let mut cube_instance = Instance::new(cube_prefab.clone());
    cube_instance.set_scale(Vector3::new(1.5, 1.5, 1.5));
    cube_instance.set_translation(Vector3::new(0.0, 0.75, -5.0));
    world.add_instance(String::from("instance1"), cube_instance);

    for i in 0..100 {
        let mut cube_instance = Instance::new(cube_prefab.clone());
        cube_instance.set_translation(Vector3::new(
            (i % 10) as f32 * 6.0,
            0.5,
            (i / 10) as f32 * 6.0,
        ));
        world.add_instance(String::from(format!("cube_{}", i)), cube_instance);
    }

    world.add_instance(
        String::from("instance2"),
        Instance::new(cube_prefab.clone()),
    );

    world.add_instance(
        String::from("instance3"),
        Instance::new(cube_prefab.clone()),
    );

    let mut floor = Instance::new(cube_prefab.clone());
    floor.set_scale(Vector3::new(80.0, 0.1, 80.0));
    floor.set_translation(Vector3::new(30.0, -0.05, 30.0));
    world.add_instance(String::from("floor"), floor);

    // DRAW STEP
    let mut step = 0;
    let mut camera_vertical_rotation = 0.0;

    world.set_update(move |device_manager, instances, camera| {
        let mut front_movement = 0.0;
        let mut side_movement = 0.0;
        let mut up_movement = 0.0;
        let rotate_horizontal = -device_manager.get_last_mouse_movement_x() * MOUSE_SENSIBILITY;
        let rotate_vertical = device_manager.get_last_mouse_movement_y() * MOUSE_SENSIBILITY;

        for key in device_manager.iter_keys() {
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
                instance.set_rotate_y(rotate_horizontal);
                instance.add_front_translation(front_movement);
                instance.add_side_translation(side_movement);
                instance.add_up_translation(up_movement);
            });

        let parent = instances.get("instance1").unwrap().clone();

        camera_vertical_rotation += rotate_vertical;
        camera_vertical_rotation = clamp(camera_vertical_rotation, -20.0, 40.0);

        let mut camera_instance = parent.clone();
        camera_instance.set_rotate_x(camera_vertical_rotation);
        camera_instance.add_front_translation(-10.0);
        camera.set_parent(&camera_instance);

        instances
            .entry(String::from("instance2"))
            .and_modify(|instance| {
                instance.reset_transform();
                instance.set_scale(Vector3::new(0.5, 0.5, 0.5));
                instance.set_parent(&parent);
                instance.set_translation(
                    SphereVector::new(1.5, -20.0, (step * 5) as f32).to_cartesian(),
                );
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
            next_frame_time = std::time::Instant::now();
            world.draw_update();
        }

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Resized(new_size) => {
                    world.change_perspective_ratio(new_size.width as f32 / new_size.height as f32)
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    world.update_key_manager(&input)
                }
                _ => (),
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    world.update_mouse_motion(delta)
                }
                glutin::event::DeviceEvent::MouseWheel { delta } => world.update_mouse_wheel(delta),
                _ => (),
            },
            _ => (),
        }
    });
}
