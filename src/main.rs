#![allow(unused_mut, unused_imports)]

use std::env::args;
use corewars_core::load_file::{Instruction};
use macroquad::prelude::*;
use egui::*;
use egui_macroquad::*;

mod sim;

fn print_core(core: &Vec<Instruction>) {
    for instruction in core {
        println!("{}", instruction.to_string());
    }
}

#[macroquad::main("eMARS")]
async fn main() {
    let args: Vec<String> = args().collect();
    let mut core: Vec<Instruction>;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => core = sim::init(args[1].clone(), args[2].clone(), 8000),
        4 => core = sim::init(args[1].clone(), args[2].clone(), match args[3].parse::<isize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")}),
        _ => panic!("Too many arguments")
    }

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {

            egui::Window::new("Hello World!").show(egui_ctx, |ui|{
                ui.label("and hello down here too");
            });

        });
    
        egui_macroquad::draw();
        next_frame().await;
    }
}