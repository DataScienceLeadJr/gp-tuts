#![allow(unused_imports)]
use std::io::stdout;
use crossterm::{
    execute,
    style::Print,
};

mod stage1;
mod stage2;

const LATEST_COMPLETED_STAGE: usize = 1;

pub const GLIUM: &str = "glium";

pub fn entrypoint(stage: Option<usize>) {
    if stage.is_none() {
        execute!(stdout(), Print(format!("defaulting to latest completed stage: {}", LATEST_COMPLETED_STAGE))).ok();
        stage1::run();
    } else {
        execute!(stdout(), Print(format!("doing glium tutorial stage {}! :D", stage.unwrap()))).ok();
    }
}