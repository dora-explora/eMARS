#![allow(unused_mut)]

use std::env::args;
// use std::time::{Duration, Instant};
// use std::thread::sleep;
use crate::sim::{Instruction, Field, Process};
use corewars_core::load_file::{Opcode, AddressMode, Modifier};
use std::collections::VecDeque;

mod sim;
mod gui;

pub(crate) struct EmarsApp {
    pub(crate) core: Vec<Instruction>, // the core.
    pub(crate) coresize: usize, // stores the size of the core, usually 8000 cells
    pub(crate) default_instruction: Instruction, // stores the default instruction for the core, usually DAT.F #0, #0
    pub(crate) core_view_size: usize, // stores the visual size of the core view
    pub(crate) teams_process_queues: Vec<VecDeque<Process>>, // contains each teams process queue in order
    pub(crate) turn: usize, // stores which teams turn it is
    pub(crate) playing: bool, // stores whether the simulation is running
    pub(crate) play_speed: f64, // stores the speed when playing as the seconds of delay per full step
}

// fn print_core(core: &Vec<Instruction>) {
//     for instruction in core {
//         println!("{}", instruction.to_string());
//     }
// }

fn load_core(args: Vec<String>, default_instruction: Instruction) -> (usize, (Vec<Instruction>, Vec<VecDeque<Process>>)) {
    let coresize: usize;
    match args.len() {
        ..=2 => panic!("Not enough arguments"),
        3 => coresize = 8000,
        4 => coresize = match args[3].parse::<usize>() { Ok(n) => n, Err(e) => panic!("Could not parse coresize argument: {e}")},
        _ => panic!("Too many arguments")
    }
    return (coresize, sim::init(args[1].clone(), args[2].clone(), coresize, default_instruction));
}

// const FRAMETIME: f64 = 1./60.;
impl eframe::App for EmarsApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        // let now = Instant::now();
        gui::core_view(self, context);
        gui::sim_manager(self, context);

        // let elapsed = now.elapsed().as_secs_f64();
        // if FRAMETIME > elapsed { sleep(Duration::from_secs_f64(FRAMETIME) - Duration::from_secs_f64(elapsed)) }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let default_instruction: Instruction = Instruction {
        opcode: Opcode::Dat,
        modifier: Modifier::F, 
        field_a: Field {
            address_mode: AddressMode::Immediate,
            value: 0,
        },
        field_b: Field {
            address_mode: AddressMode::Immediate,
            value: 0,
        }
    };
    
    let (mut coresize, (mut core, mut teams_process_queue)) = load_core(args, default_instruction);
    let turn = 0;

    let core_view_size = 2;
    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_title("eMARS").with_maximized(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(EmarsApp { core, coresize, default_instruction, core_view_size, teams_process_queues: teams_process_queue, turn , playing: false, play_speed: 1.})))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}