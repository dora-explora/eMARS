#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{Opcode, Field, Instruction};
use corewars_parser as parser;
use std::fs::read_to_string;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Process {
    pub(crate) team: u8,
    pub(crate) pointer: usize
}

pub fn init(warrior_a_path: String, warrior_b_path: String, coresize: isize) -> (Vec<Instruction>, Vec<Process>) {
    // if coresize <= 400 { panic!("Core too small") }

    let warrior_a_file_string= read_to_string(warrior_a_path.as_str()).expect("Could not find/access Warrior A's file");
    let warrior_a = match corewars_parser::parse(warrior_a_file_string.as_str()) { // this one's yoinked straight from the source
        parser::Result::Ok(warrior, warnings) => { 
            println!("Warrior A warnings: {:?}", &warnings);
            Ok(warrior)
        }
        parser::Result::Err(warrior, warnings) => {
            println!("Warrior A warnings: {:?}", &warnings);
            Err(warrior)
        }
    }.unwrap();

    let warrior_b_file_string= read_to_string(warrior_b_path.as_str()).expect("Could not find/access Warrior B's file");
    let warrior_b = match corewars_parser::parse(warrior_b_file_string.as_str()) { // this one's yoinked straight from the source
        parser::Result::Ok(warrior, warnings) => { 
            println!("Warrior B warnings: {:?}", &warnings);
            Ok(warrior)
        }
        parser::Result::Err(warrior, warnings) => {
            println!("Warrior B warnings: {:?}", &warnings);
            Err(warrior)
        }
    }.unwrap();

    // let empty_instruction = Instruction::new(Opcode::Dat, Field::immediate(0), Field::immediate(0));
    let empty_instruction = Instruction::default();
    let mut core = vec![empty_instruction; coresize as usize];

    let warrior_a_origin: isize = match warrior_a.program.origin {Some(n) => n as isize, None => 0};
    let warrior_a_instructions = warrior_a.program.instructions;
    let mut signed_index: isize;
    let mut index: usize;
    for i in 0..warrior_a_instructions.len() {
        signed_index = i as isize - warrior_a_origin;
        while signed_index.is_negative() { signed_index += coresize; }
        index = (signed_index % coresize) as usize;
        core[index] = warrior_a_instructions[i].clone();
    }
    let process_a = Process { team: 0, pointer: warrior_a_origin as usize };

    let mut rng = rand::rng();
    let warrior_b_origin: isize = match warrior_b.program.origin {Some(n) => n as isize, None => 0}; 
    let warrior_b_lower_bound = (warrior_a_instructions.len() as isize - warrior_a_origin + coresize/80) as usize;
    let warrior_b_upper_bound = (coresize - warrior_a_origin - coresize/80) as usize;
    let warrior_b_offset = rng.random_range(warrior_b_lower_bound..warrior_b_upper_bound) as isize;
    let warrior_b_instructions = warrior_b.program.instructions;
    for i in 0..warrior_b_instructions.len() {
        signed_index = i as isize - warrior_b_origin + warrior_b_offset;
        while signed_index.is_negative() { signed_index += coresize; }
        index = (signed_index % coresize) as usize;
        core[index] = warrior_b_instructions[i].clone();
    }
    let process_b = Process { team: 1, pointer: (warrior_b_offset) as usize };

    return (core, vec![process_a, process_b]);
}

pub fn step(old_core: Vec<Instruction>, old_processes: Vec<Process>, teams: u8) -> (Vec<Instruction>, Vec<Process>) {
    let mut new_core = old_core.clone();
    let mut new_processes = old_processes.clone();
    
    for team in 0..teams {
        for process in &old_processes {
            if process.team == team {
                let instruction = &old_core[process.pointer];

                if instruction.opcode == Opcode::Dat {

                } else if instruction.opcode == Opcode::Mov {
                    
                } else if instruction.opcode == Opcode::Add {
                    
                } else if instruction.opcode == Opcode::Sub {
                    
                } else if instruction.opcode == Opcode::Mul {
                    
                } else if instruction.opcode == Opcode::Div {
                    
                } else if instruction.opcode == Opcode::Mod {
                    
                } else if instruction.opcode == Opcode::Jmp {
                    
                } else if instruction.opcode == Opcode::Jmz {
                    
                } else if instruction.opcode == Opcode::Djn {
                    
                } else if instruction.opcode == Opcode::Spl {
                    
                } else if instruction.opcode == Opcode::Cmp || instruction.opcode == Opcode::Seq {
                    
                } else if instruction.opcode == Opcode::Sne {
                    
                } else if instruction.opcode == Opcode::Slt {
                    
                // } else if instruction.opcode == Opcode::Ldp { // excuse me corewa.rs??
                    
                // } else if instruction.opcode == Opcode::Sdp {
                    
                } else if instruction.opcode == Opcode::Nop {
                    
                }
                break;
            }
        }
    }

    return (new_core, new_processes);
}