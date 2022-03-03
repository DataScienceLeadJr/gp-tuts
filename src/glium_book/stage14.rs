#![allow(dead_code)]
use std::{hash::Hasher, io::Cursor};

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode},
}, buffer, uniform, Frame, texture::Texture2dDataSource};
use glium::{
    implement_vertex,
    Display,
    backend::Facade,
    Program,
    Surface,
};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

pub fn vertex_shader_src() -> &'static str {
    r#"
        #version 150

        in vec3 position;
        in vec3 normal;
        in vec2 tex_coords;

        out vec3 v_normal;
        out vec3 v_position;
        out vec2 v_tex_coords;

        uniform mat4 perspective;
        uniform mat4 view;
        uniform mat4 model;

        void main() {
            mat4 modelview = view * model;
            v_normal = transpose(inverse(mat3(modelview))) * normal;
            gl_Position = perspective * modelview * vec4(position, 1.0);
            v_position = gl_Position.xyz / gl_Position.w;
            v_tex_coords = tex_coords;
        }
    "#
}

pub fn fragment_shader_src() -> &'static str {
    r#"
        #version 140

        in vec3 v_normal;
        in vec3 v_position;
        in vec2 v_tex_coords;

        out vec4 color;

        uniform vec3 u_light;
        uniform sampler2D diffuse_tex;
        uniform sampler2D normal_tex;

        const vec3 specular_color = vec3(1.0, 0.975, 0.925);

        mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
            vec3 dp1 = dFdx(pos);
            vec3 dp2 = dFdy(pos);
            vec2 duv1 = dFdx(uv);
            vec2 duv2 = dFdy(uv);

            vec3 dp1perp = cross(normal, dp1);
            vec3 dp2perp = cross(dp2, normal);

            vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
            vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;
            
            float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
            return mat3(T * invmax, B * invmax, normal);
        }

        void main() {
            vec3 normal_map = texture(normal_tex, v_tex_coords).rgb;
            mat3 tbn = cotangent_frame(v_normal, v_position, v_tex_coords);
            vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));
            float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(u_light) + camera_dir); // relationship between lightsource and camera angle for the object/fragment

            // dot = cosine
            float specular = pow(max(dot(half_direction, normalize(real_normal)), 0.0), 16.0); // 16 = specular coefficient, determining the "dropoff rate" for specular reflectio.
            
            vec3 diffuse_color = texture(diffuse_tex, v_tex_coords).rgb;
            vec3 ambient_color = diffuse_color * 0.1;
            color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
        }
    "#
}

pub fn the_stage13_program(display: &Display) -> Program {
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

    let image = image::load(Cursor::new(&include_bytes!("D:\\Projects\\Rust\\gp-tuts\\assets\\textures\\tuto-14-diffuse.jpg")),
                                            image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_texture = glium::texture::texture2d::Texture2d::new(&display, image).unwrap();


    let nm_image = image::load(Cursor::new(&include_bytes!("D:\\Projects\\Rust\\gp-tuts\\assets\\textures\\tuto-14-normal.png")),
                                                image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = nm_image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&nm_image.into_raw(), image_dimensions);
    let normal_map = glium::texture::texture2d::Texture2d::new(&display, image).unwrap();

    // This defines 2 triangles IF we use it in a "triangle strip" index!
    let quad = glium::vertex::VertexBuffer::new(&display, &[
            Vertex { position: [-1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0] },
            Vertex { position: [ 1.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0] },
            Vertex { position: [-1.0,-1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0] },
            Vertex { position: [ 1.0,-1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0] },
        ]).unwrap();

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
            [0.8, 0.0, 0.0, 0.0],
            [0.0,0.8, 0.0, 0.0],
            [0.0, 0.0, 0.8, 0.0],
            [0.0, 0.0, 2.5, 1.0f32],
        ];

        let view = view_matrix(
            &[0.8, 0.4, 0.6],
            &[-0.4, -0.2, 1.0],
            &[0.0, 1.0, 0.0]
        );

        let uniforms = uniform! {
            u_light: [-1.0, 0.8, 0.9f32],
            model: model,
            view: view,
            perspective: perspective_matrix(&frame),
            diffuse_tex: &diffuse_texture,
            normal_tex: &normal_map
        };

        // from here on we're finally getting into all of this! :D
        // well no... we really didn't.. :/
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess, // These two lines define that each fragment's depth
                write: true, // has to be less than the already buffered depth to be written into the buffer over the previous one.
                ..Default::default()
            },
            // stage 11: backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise, NOT APPLIED FOR THE TEAPOT BECAUSE IT IS NOT A "CLOSED" MODEL. (meaning the inside potentially has to "exist")
            ..Default::default()
        };

        // Drawing the Quad!
        frame.draw(
            &quad,
            glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            &the_stage13_program(&display),
            &uniforms,
            &params,
        ).unwrap();

        frame.finish().unwrap();

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}