#![allow(unused_mut, unused_imports)]

use std::env::args;
use std::cmp::max;
use std::fmt::format;
use corewars_core::load_file::*;
use eframe::egui;
use egui::*;

mod sim;

// fn print_core(core: &Vec<Instruction>) {
//     for instruction in core {
//         println!("{}", instruction.to_string());
//     }
// }

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
    coresize: isize,
    core_view_size: usize
}

fn instruction_color(instruction: &Instruction) -> Color32 {
    if *instruction == Instruction::default() {
        return Color32::DARK_GRAY;
    } else {
        return Color32::RED;
    }
}

fn label_color(y: f32, square_size: f32) -> Color32 {
    if (y/square_size).floor() % 4. == 0. {
        return Color32::LIGHT_GRAY;
    } else if (y/square_size).floor() % 2. == 0. {
        return Color32::GRAY;
    } else {
        return Color32::DARK_GRAY;
    }
}

impl eframe::App for EmarsApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        let default_size: Vec2;
        let label_font_size: f32;
        let square_size_inside: f32;
        let square_size_outside: f32;
        let x_margin: f32;
        let y_margin: f32;
        match self.core_view_size {
            0 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin) = (vec2(519., 400.), 6., 4., 5., 20., 0.),
            1 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin) = (vec2(822., 639.), 8., 6., 8., 24., 1.),
            2 => (default_size, label_font_size, square_size_inside, square_size_outside, x_margin, y_margin) = (vec2(1026., 801.), 10., 8., 10., 28., 2.),
            _ => panic!("Invalid core view size of {}", self.core_view_size)
        };
        egui::Window::new("Core View")
        .default_size(default_size)
        .show(context, |ui|{
            
            let painter_width = ui.available_width();
            let painter_height = max((((self.coresize * 5) as f32 / painter_width).ceil() + 5.) as usize, ui.available_height() as usize) as f32;
            
            let (response, painter) = ui.allocate_painter(vec2(painter_width, painter_height), Sense::hover());
            let window_width = response.rect.width();
            let window_height = response.rect.height();
            // println!("{window_width} x {window_height}s");

            painter.text(
                pos2(response.rect.min.x, response.rect.min.y),
                Align2::LEFT_TOP,
                "0000",
                FontId::monospace(label_font_size),
                Color32::LIGHT_GRAY
            );
            

            let mut x = response.rect.min.x + x_margin; // calculates *objective* x position
            let mut y = response.rect.min.y + y_margin; // calculates *objective* y position

            for i in 0..self.coresize {
                // draws the rectangle at pos (x, y) and size (4, 4) in red
                painter.rect_filled(
                    Rect::from_min_size(pos2(x, y), vec2(square_size_inside, square_size_inside)), 
                    CornerRadius::same(0), 
                    instruction_color(&self.core[i as usize])
                );
                // moves next square's x 5 pixels to the left
                x += square_size_outside;
                if (x - response.rect.min.x) > window_width - square_size_inside { // if next square will overflow,
                    x = response.rect.min.x + x_margin; // set it's x back to the beginning
                    y += square_size_outside; // and move it down a row.

                    if i != 7999 { painter.text(
                        pos2(response.rect.min.x, y - 2.),
                        Align2::LEFT_TOP,
                        format!("{:04}", i + 1),
                        FontId::monospace(label_font_size),
                        label_color(y- response.rect.min.y, square_size_outside)
                    ); }

                    if (y - response.rect.min.y) > window_height - square_size_inside { // if next row will overflow,
                        // println!("boo");
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
    let core_view_size = 1;

    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_title("eMARS").with_maximized(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(EmarsApp { core, coresize, core_view_size })))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}