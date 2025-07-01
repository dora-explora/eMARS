#![allow(unused_mut, unused_imports)]

use std::env::args;
use corewars_core::load_file::*;

mod sim;
mod gui;

// fn print_core(core: &Vec<Instruction>) {
//     for instruction in core {
//         println!("{}", instruction.to_string());
//     }
// }

fn load_core(args: Vec<String>) -> (isize, (Vec<Instruction>, Vec<sim::Process>)) {
    let coresize: isize;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => coresize = 8000,
        4 => coresize = match args[3].parse::<isize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")},
        _ => panic!("Too many arguments")
    }
    return (coresize, sim::init(args[1].clone(), args[2].clone(), coresize));
}

impl eframe::App for gui::EmarsApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        gui::core_view(self, context);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let (mut coresize, (mut core, mut processes)) = load_core(args);

    let core_view_size = 2;
    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_title("eMARS").with_maximized(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(gui::EmarsApp { core, coresize, core_view_size, teams: processes.len() as u8, processes })))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}