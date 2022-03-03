
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
mod learn_wgpu;

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

    let args = Arg::parse();

    match args.tutorial.to_lowercase().as_str() {
        glium_book::GLIUM => glium_book::entrypoint(args.stage),
        learn_wgpu::LEARN_WGPU => glium_book::entrypoint(args.stage),
        _ => panic!("That's not even a tutorial! For God's Sake!..")
    };

    Ok(())
}
