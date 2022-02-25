
use clap::Parser;
use crossterm::{
    ExecutableCommand,
    terminal::{
        Clear,
        ClearType,
    },
    cursor,
};

mod glium_book;

/// Collective main entrypoint for running different graphics programming tutorial stages
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arg {
    /// Which tutorial to run.
    #[clap(short, long)]
    tutorial: String,

    /// identify stage to run in given tutorial.
    /// defaults to latest completed stage for given tutorial.
    #[clap(short, long)]
    stage: Option<usize>
}

fn main() -> crossterm::Result<()>{
    std::io::stdout().execute(Clear(ClearType::All))?;
    std::io::stdout().execute(cursor::MoveTo(0,0))?;

    // TODOING: getting perlin noise up and running in this his-hey!

    use bracket_noise::prelude::*;

    let mut noise = FastNoise::seeded(21);
    noise.set_noise_type(NoiseType::PerlinFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(5);
    noise.set_fractal_gain(0.6);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(2.0);

    let wh = 250;
    let mut img = image::GrayAlphaImage::new(wh, wh);
    for x in 0..wh {
        for y in 0..wh {
            img.put_pixel(x, y, image::LumaA([255, (255.0 * noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0)) as u8]));
        }
    }

    img.save(".\\test.png").expect("failed to save to file.");

    // let args = Arg::parse();

    // match args.tutorial.to_lowercase().as_str() {
    //     glium_book::GLIUM => glium_book::entrypoint(args.stage),
    //     _ => panic!("That's not even a tutorial! For God's Sake!..")
    // };

    Ok(())
}
