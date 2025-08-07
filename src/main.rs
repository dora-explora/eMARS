#![allow(unused_mut)]

use std::env::args;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::sim::{Instruction, Field, Process};
use corewars_core::load_file::{Opcode, AddressMode, Modifier};
use std::collections::VecDeque;

mod sim;
mod gui;

pub(crate) struct EmarsApp {
    core: Vec<Instruction>, // the core.
    coresize: usize, // the size of the core, usually 8000 cells
    default_instruction: Instruction, // the default instruction for the core, usually DAT.F #0, #0
    core_view_size: usize, // the visual size of the core view
    teams_process_queues: Vec<VecDeque<Process>>, // each teams process queue in order
    turn: usize, // which teams turn it is
    playing: bool, // whether the simulation is playing
    play_delay: usize, // the number of milliseconds per step during play
    last_step: Instant, //  the time since the last step during play
    state_sender: Sender<(Vec<Instruction>, Vec<VecDeque<Process>>)>,
    state_receiver: Receiver<(Vec<Instruction>, Vec<VecDeque<Process>>)>,
    play_step_count: usize, // number of steps since play started
    play_step_limit: usize, // number of steps until tie is declared
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
        match self.state_receiver.try_recv() {
            Ok((core, queues)) => { self.core = core; self.teams_process_queues = queues; },
            Err(_) => {}
        }
        gui::core_view(self, context);
        gui::sim_manager(self, context);
        if self.playing { context.request_repaint_after(Duration::from_millis(10)) };
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
    let (play_sender, play_receiver) = channel::<(Vec<Instruction>, Vec<VecDeque<Process>>)>();
    let turn = 0;

    let core_view_size = 2;
    let app = EmarsApp {
        core,
        coresize,
        default_instruction,
        core_view_size,
        teams_process_queues: teams_process_queue,
        turn,
        playing: false,
        play_delay: 1,
        last_step: Instant::now(),
        state_sender: play_sender,
        state_receiver: play_receiver,
        play_step_count: 0,
        play_step_limit: coresize * 10,
    };

    match eframe::run_native(
        "eMARS", 
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_title("eMARS").with_maximized(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(app)))
    ) {
        Err(error) => panic!("Error while rendering UI: {error}"),
        Ok(_) => assert!(true)
    };
}