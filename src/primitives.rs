#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
implement_vertex!(Vertex, position, color);

pub struct Primitive {}

impl Primitive {
    pub fn cube() -> (Vec<Vertex>, [u16; 36]) {
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
            0, 2, 1, 1, 2, 3, 4, 5, 6, 5, 7, 6, 8, 9, 10, 9, 11, 10, 12, 14, 13, 13, 14, 15, 16,
            18, 17, 17, 18, 19, 20, 21, 22, 21, 23, 22,
        ];

        (shape, indices)
    }
}
