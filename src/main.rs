#[macro_use]
extern crate glium;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 4],
    }

    implement_vertex!(Vertex, position, color);

    let shape = vec![
        Vertex {
            position: [0.5, 0.5, 0.5],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            color: [1.0, 0.0, 0.0, 1.0],
        },

        Vertex {
            position: [0.5, 0.5, -0.5],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            color: [0.0, 1.0, 0.0, 1.0],
        },

        Vertex {
            position: [-0.5, 0.5, 0.5],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            color: [0.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            color: [0.0, 0.0, 1.0, 1.0],
        },

        Vertex {
            position: [0.5, 0.5, 0.5],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            color: [0.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            color: [0.0, 1.0, 1.0, 1.0],
        },

        Vertex {
            position: [0.5, 0.5, 0.5],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            color: [1.0, 0.0, 1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, -0.5],
            color: [1.0, 0.0, 1.0, 1.0],
        },

        Vertex {
            position: [0.5, -0.5, 0.5],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            color: [1.0, 1.0, 0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            color: [1.0, 1.0, 0.0, 1.0],
        },
    ];

    let indices: [u16; 36] = [
        0, 2, 1, 
        1, 2, 3, 
        4, 5, 6, 
        5, 7, 6,
        8, 9, 10,
        9, 11, 10,
        12, 14, 13, 
        13, 14, 15,
        16, 18, 17,
        17, 18, 19,
        20, 21, 22,
        21, 23, 22];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    let vertex_shader_src = r#"
    #version 330

    layout(location = 0) in vec3 position;
    layout(location = 1) in vec4 color;
    
    smooth out vec4 theColor;
    
    uniform vec3 offset;
    uniform mat4 perspectiveMatrix;
    
    void main()
    {
        vec4 cameraPos = vec4(position, 1.0) + vec4(offset.x, offset.y, offset.z, 0.0);
    
        gl_Position = perspectiveMatrix * cameraPos;
        theColor = color;
    } 
    "#;

    let fragment_shader_src = r#"
        #version 330

        smooth in vec4 theColor;
        
        out vec4 outputColor;
        
        void main()
        {
            outputColor = theColor;
        }    
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
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
            offset: [-0.75_f32, -0.75_f32, -1.0_f32],
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
