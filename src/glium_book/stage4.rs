use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode}
}, Program, Display, backend::Facade, uniform};
use glium::Surface;

use super::stage2::{
    first_triangle,
    buffer_a_shape,
    dummy_marker,
    Vertex,
};

pub fn vertex_shader_src() -> &'static str {
    r#"
        #version 140

        in vec2 position;

        uniform mat4 translation_matrix; // UNIFORM meaning: global variable whose value is set for a draw call. aka. draw-context const variable.
        uniform mat4 rotation_matrix;

        void main() {
            vec4 rot_pos = rotation_matrix * vec4(position, 0.0, 1.0); // order is important, remember 4x4 * 4x1 vs 4x1 * 4x4
            gl_Position = translation_matrix * rot_pos; // order is important, remember 4x4 * 4x1 vs 4x1 * 4x4
        }
    "#
}

pub fn the_stage4_program(display: &Display) -> Program {
    Program::from_source(display as &dyn Facade, vertex_shader_src(), super::stage2::fragment_shader_src(), None).unwrap()
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

    let mut t: f32 = -0.5;
    let triangle = first_triangle();
    let vertex_buffer = buffer_a_shape(&display, &triangle);

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
                    },
                    _ => return,
                },
                _ => return,
            },
            _ => (),
        }
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        // update 't'
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform!{
            translation_matrix: [ // OpenGL and glium have column-major matrices
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , t * 0.25, 0.0, 1.0f32],
            ],
            rotation_matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        
        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.9, 1.0);

        frame.draw(
            &vertex_buffer,
            &dummy_marker(),
            &the_stage4_program(&display),
            &uniforms,
            &Default::default()
        ).unwrap();
        
        frame.finish().unwrap();
    });
}