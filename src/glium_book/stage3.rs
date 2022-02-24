use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode}
};
use glium::Surface;

use super::stage2::{
    first_triangle,
    buffer_a_shape,
    dummy_marker,
    the_stage2_program, Vertex,
};

trait Anime{
    fn translate(&mut self, t: f32);
}

impl Anime for [Vertex; 3] {
    fn translate(&mut self, t: f32) {
        self[0].position[0] = -0.5 + t;
        self[1].position[0] =  0.0 + t;
        self[2].position[0] =  0.5 + t;
    }
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

    let mut triangle = first_triangle();

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

        triangle.translate(t);
        let vertex_buffer = buffer_a_shape(&display, &triangle);
        
        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.9, 1.0);

        frame.draw(
            &vertex_buffer,
            &dummy_marker(),
            &the_stage2_program(&display),
            &glium::uniforms::EmptyUniforms,
            &Default::default()
        ).unwrap();
        
        frame.finish().unwrap();
    });
}