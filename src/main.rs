#![allow(unused_mut, unused_imports)]

use std::env::args;
use corewars_core::load_file::{Instruction};
use macroquad::{prelude::*, ui::root_ui, ui::hash, ui::widgets::Window};
use egui::{*, vec2, Rect, Color32};
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

        let mut window_height = screen_height()-50.0;
        let mut window_width = screen_height()*0.8;
        
        egui_macroquad::ui(|egui_ctx| {

            egui::Window::new("Core View")
            .movable(true)
            .resizable(true)
            .show(egui_ctx, |ui|{
                let (response, painter) = ui.allocate_painter(vec2(window_width, window_height), Sense::hover());
                
                let top_left = response.rect.min;
                let mut x = 2.0 + top_left.x;
                let mut y = 2.0 + top_left.y;
                for i in 0..coresize {
                    // draws the rectangle at pos (x, y) and size (4, 4) in red
                    painter.rect_filled(
                        Rect::from_min_size(pos2(x, y), vec2(4., 4.)), 
                        Rounding::none(), 
                        Color32::RED
                    );
                    // moves next square's x 5 pixels to the left
                    x += 5.0;
                    if x > window_width - 5.0 + top_left.x { // if next square will overflow,
                        x = 2.0 + top_left.x; // set it's x back to the beginning
                        y += 5.0; // and move it down a row.
                        if y > window_height + 5.0 { // if next row will overflow,
                            break; // stop rendering the squares.
                        }
                    }
                }
            });

        });

        egui_macroquad::draw();
        next_frame().await;
    }
}