#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{AddressMode, Modifier, Field as OtherField, Instruction as OtherInstruction, Opcode, Value::Literal};
use corewars_parser as parser;
use egui::Label;
use std::fs::read_to_string;
use std::collections::{vec_deque, VecDeque};
use std::thread::{spawn, sleep};
use std::time::{Duration, Instant};
use rand::Rng;

use crate::EmarsApp;

#[derive(Clone, Copy)]
pub struct Process {
    pub(crate) team: u8,
    pub(crate) pointer: usize
}

#[derive(Clone, Copy, PartialEq)]
pub struct Field {
    pub(crate) address_mode: AddressMode,
    pub(crate) value: usize
}

#[derive(Clone, Copy, PartialEq)]
pub struct Instruction {
    pub(crate) opcode: Opcode,
    pub(crate) modifier: Modifier,
    pub(crate) field_a: Field,
    pub(crate) field_b: Field
}

fn translate_instruction(old_instruction: OtherInstruction, coresize: usize) -> Instruction {
    let field_a_value = match old_instruction.field_a.value {
        Literal(n) => negative_mod(n as isize, coresize),
        _ => panic!("corewars_core Value::Label found while translating field A to crate::sim::Field")
    };
    let field_b_value = match old_instruction.field_b.value {
        Literal(n) => negative_mod(n as isize, coresize),
        _ => panic!("corewars_core Value::Label found while translating field B to crate::sim::Field")
    };

    return Instruction {
        opcode: old_instruction.opcode,
        modifier: old_instruction.modifier,
        field_a: Field {
            address_mode: old_instruction.field_a.address_mode,
            value: field_a_value,
        },
        field_b: Field {
            address_mode: old_instruction.field_b.address_mode,
            value: field_b_value,
        }
    }
}

pub fn negative_mod(n: isize, modulus: usize) -> usize {
    let mut value = n.clone();
    while value.is_negative() {
        value += modulus as isize;
    }
    return (value % modulus as isize) as usize
}

fn decrement_mod(n: &mut usize, modulus: usize) {
    if *n == 0 { *n = modulus - 1; }
    else { *n -= 1; }
}

fn increment_mod(n: &mut usize, modulus: usize) {
    if *n == (modulus - 1) { *n = 0; }
    else { *n += 1; }
}

pub fn init(warrior_a_path: String, warrior_b_path: String, coresize: usize, default_instruction: Instruction) -> (Vec<Instruction>, Vec<VecDeque<Process>>) {
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

    let mut core = vec![default_instruction; coresize];

    let warrior_a_origin: usize = match warrior_a.program.origin {Some(n) => n as usize, None => 0};
    let warrior_a_instructions = warrior_a.program.instructions;
    let mut signed_index: isize;
    let mut index: usize;
    for i in 0..warrior_a_instructions.len() {
        core[i] = translate_instruction(warrior_a_instructions[i].clone(), coresize);
    }
    let process_a = Process { team: 0, pointer: warrior_a_origin };

    let mut rng = rand::rng();
    let warrior_b_origin: usize = match warrior_b.program.origin {Some(n) => n as usize, None => 0}; 
    let warrior_b_lower_bound = warrior_a_instructions.len() - warrior_a_origin + coresize/80;
    let warrior_b_upper_bound = coresize - warrior_a_origin - coresize/80;
    let warrior_b_offset = rng.random_range(warrior_b_lower_bound..warrior_b_upper_bound) as isize;
    let warrior_b_instructions = warrior_b.program.instructions;
    for i in 0..warrior_b_instructions.len() {
        signed_index = i as isize - warrior_b_origin as isize + warrior_b_offset;
        index = negative_mod(signed_index, coresize);
        core[index] = translate_instruction(warrior_b_instructions[i].clone(), coresize);
    }
    let process_b = Process { team: 1, pointer: warrior_b_offset as usize };

    return (core, vec![VecDeque::from([process_a]), VecDeque::from([process_b])]);
}

fn step_process(core: &mut Vec<Instruction>, coresize: usize, process_queue: &mut VecDeque<Process>) { // steps with the first process in the process queue
    
    let process = &mut process_queue[0];
    let instruction = core[process.pointer].clone();
    let mut dead: bool = false;

    // process predecrements for field a
    if instruction.field_a.address_mode == AddressMode::PreDecIndirectA {
        decrement_mod(&mut core[(instruction.field_a.value + process.pointer) % coresize].field_a.value, coresize);
    } else if instruction.field_a.address_mode == AddressMode::PreDecIndirectB {
        decrement_mod(&mut core[(instruction.field_a.value + process.pointer) % coresize].field_b.value, coresize);
    }

    // process predecrements for field b
    if instruction.field_b.address_mode == AddressMode::PreDecIndirectA {
        decrement_mod(&mut core[(instruction.field_b.value + process.pointer) % coresize].field_a.value, coresize);
    } else if instruction.field_b.address_mode == AddressMode::PreDecIndirectB {
        decrement_mod(&mut core[(instruction.field_b.value + process.pointer) % coresize].field_b.value, coresize);
    }

    // big if block for all the opcodes
    if instruction.opcode == Opcode::Dat { // kills the first process (this process)
        dead = true;
    } else if instruction.opcode == Opcode::Mov { // moves instruction/values specified by A field to instruction specified by B field
        let source_instruction_pointer: usize; // pointer to first instruction of MOV; the instruction to be copied (relative address)
        let dest_instruction_pointer: usize; // pointer to second instruction of MOV; the instruction to be copied to (relative address)

        match instruction.field_a.address_mode {
            AddressMode::Immediate => 
                source_instruction_pointer = 0,
            AddressMode::Direct => 
                source_instruction_pointer = instruction.field_a.value,
            AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA => 
                source_instruction_pointer = core[(instruction.field_a.value + process.pointer) % coresize].field_a.value + instruction.field_a.value,
            AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB => 
                source_instruction_pointer = core[(instruction.field_a.value + process.pointer) % coresize].field_b.value + instruction.field_a.value
        }

        match instruction.field_b.address_mode {
            AddressMode::Immediate => 
                dest_instruction_pointer = 0,
            AddressMode::Direct => 
                dest_instruction_pointer = instruction.field_b.value,
            AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA => 
                dest_instruction_pointer = core[(instruction.field_b.value + process.pointer) % coresize].field_a.value + instruction.field_b.value,
            AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB => 
                dest_instruction_pointer = core[(instruction.field_b.value + process.pointer) % coresize].field_b.value + instruction.field_b.value
        }

        let destination = (dest_instruction_pointer + process.pointer) % coresize;
        let source = (source_instruction_pointer + process.pointer) % coresize;
        match instruction.modifier {
            Modifier::A =>
                core[destination].field_a.value = core[source].field_a.value,
            Modifier::B =>
                core[destination].field_b.value = core[source].field_b.value,
            Modifier::AB =>
                core[destination].field_b.value = core[source].field_a.value,
            Modifier::BA =>
                core[destination].field_a.value = core[source].field_b.value,
            Modifier::F =>
                { core[destination].field_a.value = core[source].field_a.value;
                core[destination].field_b.value = core[source].field_b.value; },
            Modifier::X =>
                { core[destination].field_b.value = core[source].field_a.value;
                core[destination].field_a.value = core[source].field_b.value; },
            Modifier::I =>
                core[destination] = core[source],
        }
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
        
    };

    // process postincrements for field a
    if instruction.field_a.address_mode == AddressMode::PostIncIndirectA {
        increment_mod(&mut core[(instruction.field_a.value + process.pointer) % coresize].field_a.value, coresize);
    } else if instruction.field_a.address_mode == AddressMode::PostIncIndirectB {
        increment_mod(&mut core[(instruction.field_a.value + process.pointer) % coresize].field_b.value, coresize);
    }

    // process postincrements for field b
    if instruction.field_b.address_mode == AddressMode::PostIncIndirectA {
        increment_mod(&mut core[(instruction.field_b.value + process.pointer) % coresize].field_a.value, coresize);
    } else if instruction.field_b.address_mode == AddressMode::PostIncIndirectB {
        increment_mod(&mut core[(instruction.field_b.value + process.pointer) % coresize].field_b.value, coresize);
    }

    if dead {
        process_queue.remove(0);
    } else {
        process.pointer += 1;
        process.pointer %= coresize;
    }
}

pub fn part_step(core: &mut Vec<Instruction>, coresize: usize, teams_process_queues: &mut Vec<VecDeque<Process>>, turn: &mut usize) { // steps the team whose turn it is
    let mut process_queue = &mut teams_process_queues[*turn];
    step_process(core, coresize, process_queue);
    if process_queue.len() != 0 { 
        process_queue.rotate_left(1); 
        *turn += 1;
        *turn %= teams_process_queues.len();
    } else {
        println!("Warrior {turn} is dead!");
        teams_process_queues.remove(*turn);
        if teams_process_queues.len() >= *turn {
            *turn = 0;
        }
    }
} 

pub fn full_step(core: &mut Vec<Instruction>, coresize: usize, teams_process_queues: &mut Vec<VecDeque<Process>>, turn: &mut usize) { // steps until the turn is back to 0
    part_step(core, coresize, teams_process_queues, turn);
    while *turn != 0 && teams_process_queues.len() > 1 {
        part_step(core, coresize, teams_process_queues, turn)
    }
}

// impl EmarsApp {
//     pub fn play(&mut self) {
//         while self.playing {
//             let now = Instant::now();
//
//             full_step(&mut self.core, self.coresize, &mut self.teams_process_queues, &mut self.turn);
//
//             let elapsed = now.elapsed().as_secs_f64();
//             if self.play_speed > elapsed { sleep(Duration::from_secs_f64(self.play_speed) - Duration::from_secs_f64(elapsed)) }
//         }
//     }
// }