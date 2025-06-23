#![allow(unused_mut, unused_imports)]

use std::env::args;
use std::cmp::max;
use corewars_core::load_file::*;
use eframe::egui;
use egui::*;

mod sim;

fn print_core(core: &Vec<Instruction>) {
    for instruction in core {
        println!("{}", instruction.to_string());
    }
}

fn load_core(args: Vec<String>) -> (isize, Vec<Instruction>) {
    let coresize: isize;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => coresize = 8000,
        4 => coresize = match args[3].parse::<isize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")},
        _ => panic!("Too many arguments")
    }
    return (coresize, sim::init(args[1].clone(), args[2].clone(), coresize));
}

struct EmarsApp {
    core: Vec<Instruction>,
    coresize: isize
}

fn instruction_color(instruction: &Instruction) -> Color32 {
    if *instruction == Instruction::default() {
        return Color32::DARK_GRAY;
    } else {
        return Color32::RED;
    }
}

impl eframe::App for EmarsApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        egui::Window::new("Core View")
        .default_size(vec2(501., 402.))
        .show(context, |ui|{
            
            let painter_width = ui.available_width();
            let painter_height = max((((self.coresize * 5) as f32 / painter_width).ceil() + 5.) as usize, ui.available_height() as usize) as f32;
            
            let (response, painter) = ui.allocate_painter(vec2(painter_width, painter_height), Sense::hover());
            
            let mut x = 2. + response.rect.min.x; // calculates *objective* x position
            let mut y = 2. + response.rect.min.y; // calculates *objective* y position
            let window_width = response.rect.max.x - response.rect.min.x;
            let window_height = response.rect.max.y - response.rect.min.y;
            
            for i in 0..self.coresize {
                // draws the rectangle at pos (x, y) and size (4, 4) in red
                painter.rect_filled(
                    Rect::from_min_size(pos2(x, y), vec2(4., 4.)), 
                    CornerRadius::same(0), 
                    instruction_color(&self.core[i as usize])
                );
                // moves next square's x 5 pixels to the left
                x += 5.;
                if (x - response.rect.min.x) > window_width { // if next square will overflow,
                    x = 2. + response.rect.min.x; // set it's x back to the beginning
                    y += 5.; // and move it down a row.
                    if (y - response.rect.min.y) > window_height + 0. { // if next row will overflow,
                        println!("boo");
                        break; // stop rendering the squares.
                    }
                }
            }

        });
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let (mut coresize, mut core) = load_core(args);    
    
    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions::default(), 
        Box::new(|_cc| Ok(Box::new(EmarsApp { core, coresize })))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}