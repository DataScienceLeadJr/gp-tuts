#![allow(dead_code)]
use glium::{implement_vertex, Display, backend::Facade, Program};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn first_triangle() -> [Vertex; 3] {
    let vertex1 = Vertex { position: [-0.5, -0.5]};
    let vertex2 = Vertex { position: [ 0.0, 0.5]};
    let vertex3 = Vertex { position: [ 0.5, -0.25]};
    [vertex1, vertex2, vertex3]
}

fn buffer_a_shape(display: &Display, shape: &Vec<Vertex>) -> glium::VertexBuffer<Vertex> {
    // Takes a CPU-memory stored shape and uploads it to the video card memory.
    glium::VertexBuffer::new(display as &dyn Facade, shape).unwrap()
}

fn dummy_marker() -> glium::index::NoIndices{
    // For when you don't really need the linking of vertices.
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList)
}

fn vertex_shader_src() -> &'static str {
    r#"
        #version 140

        in vec2 position; // vec2 = [f32; 2] in Rust, name only required to match struct data field, name in-and-of-itself doesn't matter.

        void main() {
            gl_Position = vec4(position, 0.0, 1.0); // coordinates are actually 4D, x/y/z/? is this for quarternion-ing?
        }
    "#
}

fn fragment_shader_src() -> &'static str {
    // aka. pixel shader
    r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.9, 0.15, 0.1, 1.0);
        }
    "#
}

fn the_program(display: &Display) -> Program {
    Program::from_source(display as &dyn Facade, vertex_shader_src(), fragment_shader_src(), None).unwrap()
}

