#![allow(dead_code)]
use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode},
}, buffer};
use glium::{
    implement_vertex,
    Display,
    backend::Facade,
    Program,
    Surface,
};

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

fn buffer_a_shape(display: &Display, shape: &[Vertex]) -> glium::vertex::VertexBuffer<Vertex> {
    // Takes a CPU-memory stored shape and uploads it to the video card memory.
    glium::vertex::VertexBuffer::new(display as &dyn Facade, shape).unwrap()
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

pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(
        window_builder,
        context_builder,
        &event_loop
    ).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.9, 1.0);

        // Drawing the Triangle!
        frame.draw(
            &buffer_a_shape(&display, &first_triangle()[..]),
            dummy_marker(),
            &the_program(&display),
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();

        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {
                    input: KeyboardInput { virtual_keycode: Some(virtual_code), state, .. },
                    ..
                } => match (virtual_code, state) {
                    (VirtualKeyCode::Escape, _) => *control_flow = glutin::event_loop::ControlFlow::Exit,
                    _ => return,
                },
                _ => return,
            },
            _ => (),
        }
    });
}