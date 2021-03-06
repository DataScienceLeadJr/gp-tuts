use std::hash::Hasher;

use crossterm::event::KeyCode;
use glium::{glutin::{
    self,
    event::{KeyboardInput, VirtualKeyCode}
}, Program, Display, backend::Facade, uniform, implement_vertex, Blend};
use glium::Surface;
use image::EncodableLayout;

use crate::glium_book::stage2::{buffer_a_shape, dummy_marker};

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    texture_coordinates: [f32; 2],
}

implement_vertex!(Vertex, position, texture_coordinates);


pub fn texture_triangle() -> Vec<Vertex> {
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        texture_coordinates: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        texture_coordinates: [0.1, 0.8],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        texture_coordinates: [0.75, 0.12],
    };

    vec![
        vertex1,
        vertex2,
        vertex3,
    ]
}


pub fn vertex_shader_src() -> &'static str {
    r#"
        #version 140

        in vec2 position;
        in vec2 texture_coordinates; // THIS!

        out vec2 my_attr; // out variables are communication channels from vertex to fragment shader programs!
        out vec2 v_tex_coords; // OUT TO THIS!

        uniform mat4 translation_matrix; // UNIFORM meaning: global variable whose value is set for a draw call. aka. draw-context const variable.
        uniform mat4 rotation_matrix;

        void main() {
            // Need to do this so that it can be "passed along" to the fragment shader so that the pixel-in-texture-position can be interpolated from the vertex pos.
            v_tex_coords = texture_coordinates;

            // NOTE: the interpolation happens because the default for variables like this is the "smooth" setting.
            my_attr = position; // this is only the vertex position, but when the fragment shader reads it it is an interpolated value corresponding automatically to the pixels relative position! :D <3 

            vec4 rot_pos = rotation_matrix * vec4(position, 0.0, 1.0); // order is important, remember 4x4 * 4x1 vs 4x1 * 4x4
            gl_Position = translation_matrix * rot_pos; // order is important, remember 4x4 * 4x1 vs 4x1 * 4x4
        }
    "#
}

pub fn fragment_shader_src() -> &'static str {
    r#"
        #version 140

        // like the rest: it just needs to be the same as in the vertex shader.
        in vec2 my_attr; // HERE IT IS :D the interpolated fragment position the vertex shader stored for us.

        in vec2 v_tex_coords; // THIS COMING IN RIGHT HERE!

        out vec4 color;

        uniform float tex_zoom_sin;
        uniform sampler2D tex_img;
        uniform sampler2D tex_noise;

        void main() {
            // getting the sampled interpolated pixel attached to the corresponding position within the texture (image)
            vec4 c_tex = texture(tex_noise, v_tex_coords * tex_zoom_sin);

            // moving the color spectrum up above 0 to remove the black third for the attrib.
            // vec2 c_attrib = (my_attr + 0.5001) * 0.45;

            // vec2 c = c_tex.rg + (c_attrib * 0.1);
            // this just sets x to red and y to green, blue to 0.0 and opacity to 1.0
            // color = vec4(c, 0.0, 1.0); // awesome
            color = vec4(c_tex.rgb, c_tex.r);
        }
    "#
}

pub fn the_stage5_program(display: &Display) -> Program {
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

    use std::io::Cursor;
    use image::load;

    let image1 = load(Cursor::new(&include_bytes!("D:\\Projects\\Rust\\gp-tuts\\assets\\textures\\hamster.jpg")),
                        image::ImageFormat::Jpeg).unwrap().to_rgb8();
    let image_dimensions = image1.dimensions();

    let tex_image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image1.into_raw(), image_dimensions);
    let tex_img = glium::texture::texture2d::Texture2d::new(&display, tex_image).unwrap();
    
    // TODOING: getting perlin noise up and running in this his-hey!

    use bracket_noise::prelude::*;

    let mut noise = FastNoise::seeded(21);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(4);
    noise.set_fractal_gain(0.5);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(4.0);

    let wh = ((image_dimensions.0 + image_dimensions.1) as f32 / 2.0) as u32;
    let mut img = image::RgbaImage::new(wh, wh);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let n = ((255.0 * noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0)) as u8).clamp(0, 255);
        *pixel =
                image::Rgba(
                    [n,
                    n,
                    n,
                    250
                ]
            )
        ;
    }
    // img.save("test_rgba.png").unwrap();
    let noise_image = 
            glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), (wh, wh));
    let tex_noise = glium::texture::texture2d::Texture2d::new(&display, noise_image).unwrap();

    let mut t: f32 = -0.5;
    let triangle = texture_triangle();
    let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &triangle[..]).unwrap();

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
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(8_000_667);
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
                [ t , t * 0.75, 0.0, 1.0f32],
            ],
            rotation_matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos() * 1.54, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex_zoom_sin: t.sin() * 0.4 + 0.6,
            tex_img: &tex_img,
            tex_noise: &tex_noise,
        };
        
        let mut frame = display.draw();
        frame.clear_color(0.1, 0.1, 0.9, 1.0);

        frame.draw(
            &vertex_buffer,
            &dummy_marker(),
            &the_stage5_program(&display),
            &uniforms,
            &glium::DrawParameters {
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
        ).unwrap();
        
        frame.finish().unwrap();
    });
}