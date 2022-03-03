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
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 perspective;
        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#
}

pub fn fragment_shader_src() -> &'static str {
    r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;

        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.555, 0.007, 0.075);
            vec3 regular_color = vec3(1.0, 0.09, 0.045);
            // mix = lerp
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#
}

pub fn the_stage10_program(display: &Display) -> Program {
    Program::from_source(display as &dyn Facade, vertex_shader_src(), fragment_shader_src(), None).unwrap()
}

pub fn run() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24); // 24 is apparently just a "common value"
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

        let mut frame = display.draw();
        frame.clear_color_and_depth((0.06, 0.075, 0.95, 1.0), 1.0);
            
        let perspective = {
            let (width, height) = frame.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0; // user parameter
            let zfar = 1024.0; // can't move object farther or nearer than these
            let znear = 0.1;  // two values here.

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio   ,   0.0 ,           0.0     ,               0.0],
                [   0.0             ,   f   ,           0.0     ,               0.0],
                [   0.0             ,   0.0 ,    (zfar+znear)/(zfar-znear)  ,   1.0],
                [   0.0             ,   0.0 , -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let uniforms = uniform! {
            u_light: [-1.0, 0.8, 0.9f32],
            matrix: matrix,
            perspective: perspective
        };

        // from here on we're finally getting into all of this! :D
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess, // These two lines define that each fragment's depth
                write: true, // has to be less than the already buffered depth to be written into the buffer over the previous one.
                ..Default::default()
            },
            // stage 11: backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise, NOT APPLIED FOR THE TEAPOT BECAUSE IT IS NOT A "CLOSED" MODEL. (meaning the inside potentially has to "exist")
            ..Default::default()
        };

        // Drawing the Teapot!
        frame.draw(
            (&positions, &normals),
            &indices,
            &the_stage10_program(&display),
            &uniforms,
            &params,
        ).unwrap();

        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}