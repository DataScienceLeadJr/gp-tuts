#![allow(unused_imports)]
use std::io::stdout;
use crossterm::{
    execute,
    style::Print,
};

mod stage1;

const LATEST_COMPLETED_STAGE: usize = 0;

pub const LEARN_WGPU: &str = "lwgpu";

pub fn entrypoint(stage: Option<usize>) {
    if stage.is_none() {
        todo!("haven't even started yet! patience...");
        // execute!(stdout(), Print(format!("defaulting to latest completed stage: {}", LATEST_COMPLETED_STAGE))).ok();
    } else {
        let which_stage = stage.unwrap();
        execute!(stdout(), Print(format!("doing glium tutorial stage {}! :D", stage.unwrap()))).ok();
        match which_stage {
            _ => todo!("I'M WORKING ON IT!"),
        }
    }
}