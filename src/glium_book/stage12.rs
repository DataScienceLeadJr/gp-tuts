#![allow(dead_code)]
use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode},
}, buffer, uniform, Frame};
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
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
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

pub fn perspective_matrix(frame: &Frame) -> [[f32; 4]; 4] {
    
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

    perspective
}

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f_norm = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [
        up[1] * f_norm[2] - up[2] * f_norm[1],
        up[2] * f_norm[0] - up[0] * f_norm[2],
        up[0] * f_norm[1] - up[1] * f_norm[0],
    ];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [
        f_norm[1] * s_norm[2] - f_norm[2] * s_norm[1],
        f_norm[2] * s_norm[0] - f_norm[0] * s_norm[2],
        f_norm[0] * s_norm[1] - f_norm[1] * s_norm[0],
    ];

    let p = [
        -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
        -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
        -position[0] * f_norm[0] - position[1] * f_norm[1] - position[2] * f_norm[2],
    ];

    [
        [s_norm[0], u[0], f_norm[0], 0.0],
        [s_norm[1], u[1], f_norm[1], 0.0],
        [s_norm[2], u[2], f_norm[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
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

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let view = view_matrix(
            &[2.0, -1.0, 1.0],
            &[-2.0, 1.0, 1.0],
            &[0.0, 1.0, 0.0]
        );

        let uniforms = uniform! {
            u_light: [-1.0, 0.8, 0.9f32],
            model: model,
            view: view,
            perspective: perspective_matrix(&frame)
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

        // Drawing the Triangle!
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