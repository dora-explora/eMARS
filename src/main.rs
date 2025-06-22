#![allow(unused_mut, unused_imports)]

use std::env::args;
use corewars_core::load_file::{Instruction};
use macroquad::{prelude::*, prelude::vec2, ui::root_ui, ui::hash, math::Rect, ui::widgets::Window};
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
    let coresize: isize;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => coresize = 8000,
        4 => coresize = match args[3].parse::<isize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")},
        _ => panic!("Too many arguments")
    }
    core = sim::init(args[1].clone(), args[2].clone(), coresize);

    loop {
        clear_background(WHITE);

        let window_height = screen_height()-20.0;
        let window_width = screen_height()*0.8-20.0;
        Window::new(hash!(), vec2(0.0, 0.0), vec2(window_width, window_height),)
        .titlebar(true)
        .label("Core View")
        .movable(true)
        .ui(&mut *root_ui(), |ui| {
            let mut canvas = ui.canvas();

            let mut x = 2.0;
            let mut y = 2.0;
            for i in 0..coresize {
                canvas.rect(Rect::new(x, y, 4.0, 4.0), RED, RED);
                x += 5.0;
                if x > window_width - 5.0 {
                    x = 2.0;
                    y += 5.0;
                    if y > window_height + 5.0 {
                        break;
                    }
                }
            }
        });

        // egui_macroquad::ui(|egui_ctx| {

        //     egui::Window::new("Hello World!").show(egui_ctx, |ui|{
        //         ui.label("and hello down here too");
        //     });

        // });
        // egui_macroquad::draw();

        next_frame().await;
    }
}