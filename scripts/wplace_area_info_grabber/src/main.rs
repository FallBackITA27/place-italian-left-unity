use std::{fs::OpenOptions, io::Write};

use crate::inputs_processor::{MainLoop, UserData};

mod inputs_processor;

#[tokio::main]
async fn main() {
    let inputs = include_str!("../areas");
    let inputs = inputs_processor::Inputs::read(inputs);

    let mut looper = MainLoop::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("./out.md")
            .expect("Couldn't open file"),
    );

    for input in inputs {
        looper.generate(input).await;
    }
}
