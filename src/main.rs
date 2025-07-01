#![allow(unused_mut)]

use std::env::args;
use crate::sim::{Instruction, Field};
use corewars_core::load_file::{Opcode, AddressMode, Modifier};

mod sim;
mod gui;

// fn print_core(core: &Vec<Instruction>) {
//     for instruction in core {
//         println!("{}", instruction.to_string());
//     }
// }

fn load_core(args: Vec<String>, default_instruction: Instruction) -> (isize, (Vec<Instruction>, Vec<sim::Process>)) {
    let coresize: isize;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => coresize = 8000,
        4 => coresize = match args[3].parse::<isize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")},
        _ => panic!("Too many arguments")
    }
    return (coresize, sim::init(args[1].clone(), args[2].clone(), coresize, default_instruction));
}

impl eframe::App for gui::EmarsApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        gui::core_view(self, context);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    const DEFAULT_INSTRUCTION: Instruction = Instruction {
        opcode: Opcode::Dat,
        modifier: Modifier::F, 
        field_a: Field {
            address_mode: AddressMode::Direct,
            value: 0,
        },
        field_b: Field {
            address_mode: AddressMode::Direct,
            value: 0,
        }
    };
    let (mut coresize, (mut core, mut processes)) = load_core(args, DEFAULT_INSTRUCTION);
    

    let core_view_size = 2;
    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_title("eMARS").with_maximized(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(gui::EmarsApp { core, coresize, default_instruction: DEFAULT_INSTRUCTION, core_view_size, teams: processes.len() as u8, processes })))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}