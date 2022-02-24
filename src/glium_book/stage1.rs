use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode}
};
use glium::Surface;

pub fn run() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
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