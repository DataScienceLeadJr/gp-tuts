#![allow(dead_code)]
use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode},
}, buffer, uniform};
use glium::{
    implement_vertex,
    Display,
    backend::Facade,
    Program,
    Surface,
};

use super::teapot;

pub fn vertex_shader_src() -> &'static str {
    r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0); // the position is 4D because: the window space is 3D, and the 3 dimensions are divided by the 4th after the shader has been executed, and is then discarded.
        }
    "#
}

pub fn fragment_shader_src() -> &'static str {
    // aka. pixel shader
    r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.98, 0.15, 0.1, 1.0);
        }
    "#
}

pub fn the_stage7_program(display: &Display) -> Program {
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

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    event_loop.run(move |event, _, control_flow| {

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
                    (VirtualKeyCode::Escape, _) => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    _ => return,
                },
                _ => return,
            },
            _ => (),
        }

        let matrix = [
            [0.0065, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let uniforms = uniform! {
            matrix: matrix,
        };

        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.9, 1.0);

        // Drawing the Teapot!
        frame.draw(
            (&positions, &normals),
            &indices,
            &the_stage7_program(&display),
            &uniforms,
            &Default::default()
        ).unwrap();

        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}