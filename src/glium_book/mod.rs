#![allow(unused_imports)]
use std::io::stdout;
use crossterm::{
    execute,
    style::Print,
};

mod stage1;
mod stage2;
mod stage3;
mod stage4;
mod stage5;
mod stage6;
mod stage7;
mod stage8;
mod stage9;
mod stage10_w_11;
mod stage12;
mod stage13;
mod stage14;

mod teapot;

const LATEST_COMPLETED_STAGE: usize = 13;

pub const GLIUM: &str = "glium";

pub fn entrypoint(stage: Option<usize>) {
    if stage.is_none() {
        execute!(stdout(), Print(format!("defaulting to latest completed stage: {}", LATEST_COMPLETED_STAGE))).ok();
        stage13::run();
    } else {
        let which_stage = stage.unwrap();
        execute!(stdout(), Print(format!("doing glium tutorial stage {}! :D", stage.unwrap()))).ok();
        match which_stage {
            1 => stage1::run(),
            2 => stage2::run(),
            3 => stage3::run(),
            4 => stage4::run(),
            5 => stage5::run(),
            6 => stage6::run(),
            7 => stage7::run(),
            8 => stage8::run(),
            9 => stage9::run(),
            10 | 11 => stage10_w_11::run(),
            12 => stage12::run(),
            13 => stage13::run(),
            14 => stage14::run(),
            _ => todo!("I'M WORKING ON IT!"),
        }
    }
}