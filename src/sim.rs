#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{AddressMode, Modifier, Field as OtherField, Instruction as OtherInstruction, Opcode, Value::Literal};
use corewars_parser as parser;
use egui::Label;
use std::fs::read_to_string;
use std::collections::{vec_deque, VecDeque};
use std::thread::{spawn, sleep};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
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

fn negative_mod(n: isize, modulus: usize) -> usize {
    let mut value = n.clone();
    while value.is_negative() {
        value += modulus as isize;
    }
    return (value % modulus as isize) as usize
}

fn minus_mod(a: usize, b: usize, modulus: usize) -> usize {
    return if b > a {
        a + modulus - b
    } else {
        a - b
    }
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

fn calculate_source_and_dest_pointers(instruction: &Instruction, core: &Vec<Instruction>, process_pointer: usize, coresize: usize) -> (usize, usize) {
    let source_instruction_pointer: usize; // pointer to first instruction of MOV; the instruction to be copied (relative address)
    let dest_instruction_pointer: usize; // pointer to second instruction of MOV; the instruction to be copied to (relative address)

    match instruction.field_a.address_mode {
        AddressMode::Immediate =>
            source_instruction_pointer = 0,
        AddressMode::Direct =>
            source_instruction_pointer = instruction.field_a.value,
        AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA =>
            source_instruction_pointer = core[(instruction.field_a.value + process_pointer) % coresize].field_a.value + instruction.field_a.value,
        AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB =>
            source_instruction_pointer = core[(instruction.field_a.value + process_pointer) % coresize].field_b.value + instruction.field_a.value
    }

    match instruction.field_b.address_mode {
        AddressMode::Immediate =>
            dest_instruction_pointer = 0,
        AddressMode::Direct =>
            dest_instruction_pointer = instruction.field_b.value,
        AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA =>
            dest_instruction_pointer = core[(instruction.field_b.value + process_pointer) % coresize].field_a.value + instruction.field_b.value,
        AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB =>
            dest_instruction_pointer = core[(instruction.field_b.value + process_pointer) % coresize].field_b.value + instruction.field_b.value
    }

    return (source_instruction_pointer, dest_instruction_pointer);
}

fn step_process(core: &mut Vec<Instruction>, coresize: usize, process_queue: &mut VecDeque<Process>) { // steps with the first process in the process queue
    let process = process_queue[0];
    let instruction = core[process.pointer].clone();
    let mut dead: bool = false;
    let mut step: bool = true;

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

    let (source_instruction_pointer, dest_instruction_pointer) = calculate_source_and_dest_pointers(&instruction, &core, process.pointer, coresize);
    let source = (source_instruction_pointer + process.pointer) % coresize;
    let destination = (dest_instruction_pointer + process.pointer) % coresize;

    // big if block for all the opcodes
    match instruction.opcode {
        Opcode::Dat => { // kills the first process (this process)
            dead = true;
        }
        Opcode::Mov => { // moves instruction/values specified by A field to instruction specified by B field
            match instruction.modifier {
                Modifier::A =>
                    core[destination].field_a.value = core[source].field_a.value,
                Modifier::B =>
                    core[destination].field_b.value = core[source].field_b.value,
                Modifier::AB =>
                    core[destination].field_b.value = core[source].field_a.value,
                Modifier::BA =>
                    core[destination].field_a.value = core[source].field_b.value,
                Modifier::F => {
                    core[destination].field_a.value = core[source].field_a.value;
                    core[destination].field_b.value = core[source].field_b.value; },
                Modifier::X => {
                    core[destination].field_b.value = core[source].field_a.value;
                    core[destination].field_a.value = core[source].field_b.value; },
                Modifier::I =>
                    core[destination] = core[source],
            }
        }
        Opcode::Add => { // adds number(s) specified by A field to instruction specified by B field
            match instruction.modifier {
                Modifier::A => {
                    core[destination].field_a.value += core[source].field_a.value;
                    core[destination].field_a.value %= coresize;
                }
                Modifier::B => {
                    core[destination].field_b.value += core[source].field_b.value;
                    core[destination].field_b.value %= coresize;}
                Modifier::AB => {
                    core[destination].field_b.value += core[source].field_a.value;
                    core[destination].field_b.value %= coresize;
                }
                Modifier::BA => {
                    core[destination].field_a.value += core[source].field_b.value;
                    core[destination].field_a.value %= coresize;
                }
                Modifier::F | Modifier::I => {
                    core[destination].field_a.value += core[source].field_a.value;
                    core[destination].field_a.value %= coresize;
                    core[destination].field_b.value += core[source].field_b.value;
                    core[destination].field_b.value %= coresize;
                }
                Modifier::X => {
                    core[destination].field_a.value += core[source].field_b.value;
                    core[destination].field_a.value %= coresize;
                    core[destination].field_b.value += core[source].field_a.value;
                    core[destination].field_b.value %= coresize;
                }
            }
        }
        Opcode::Sub => { // subtracts number(s) specified by A field from instruction specified by B field
            match instruction.modifier {
                Modifier::A => {
                    core[destination].field_a.value = minus_mod(core[destination].field_a.value, core[source].field_a.value, coresize);
                }
                Modifier::B => {
                    core[destination].field_b.value = minus_mod(core[destination].field_b.value, core[source].field_b.value, coresize);
                }
                Modifier::AB => {
                    core[destination].field_b.value = minus_mod(core[destination].field_b.value, core[source].field_a.value, coresize);
                }
                Modifier::BA => {
                    core[destination].field_a.value = minus_mod(core[destination].field_a.value, core[source].field_b.value, coresize);
                }
                Modifier::F | Modifier::I => {
                    core[destination].field_a.value = minus_mod(core[destination].field_a.value, core[source].field_a.value, coresize);
                    core[destination].field_b.value = minus_mod(core[destination].field_b.value, core[source].field_b.value, coresize);
                }
                Modifier::X => {
                    core[destination].field_a.value = minus_mod(core[destination].field_a.value, core[source].field_b.value, coresize);
                    core[destination].field_b.value = minus_mod(core[destination].field_b.value, core[source].field_a.value, coresize);
                }
            }
        }
        Opcode::Mul => { // multiplies number(s) specified by A field into instruction specified by B field
            match instruction.modifier {
                Modifier::A => {
                    core[destination].field_a.value *= core[source].field_a.value;
                    core[destination].field_a.value %= coresize;
                },
                Modifier::B => {
                    core[destination].field_b.value *= core[source].field_b.value;
                    core[destination].field_b.value %= coresize;

                },
                Modifier::AB => {
                    core[destination].field_b.value *= core[source].field_a.value;
                    core[destination].field_b.value %= coresize;

                },
                Modifier::BA => {
                    core[destination].field_a.value *= core[source].field_b.value;
                    core[destination].field_a.value %= coresize;
                },
                Modifier::F | Modifier::I => {
                    core[destination].field_a.value *= core[source].field_a.value;
                    core[destination].field_a.value %= coresize;
                    core[destination].field_b.value *= core[source].field_b.value;
                    core[destination].field_b.value %= coresize;
                },
                Modifier::X => {
                    core[destination].field_b.value *= core[source].field_a.value;
                    core[destination].field_b.value %= coresize;
                    core[destination].field_a.value *= core[source].field_b.value;
                    core[destination].field_a.value %= coresize;
                }
            }
        }
        Opcode::Div => { // divides instruction specified by B field by number(s) specified by A field
            match instruction.modifier {
                Modifier::A => {
                    if core[source].field_a.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value /= core[source].field_a.value;
                    }
                },
                Modifier::B => {
                    if core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value /= core[source].field_b.value;
                    }
                },
                Modifier::AB => {
                    if core[source].field_a.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value /= core[source].field_a.value;
                    }
                },
                Modifier::BA => {
                    if core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value /= core[source].field_b.value;
                    }
                },
                Modifier::F | Modifier::I => {
                    if core[source].field_a.value == 0 || core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value /= core[source].field_a.value;
                        core[destination].field_b.value /= core[source].field_b.value;
                    }
                },
                Modifier::X => {
                    if core[source].field_a.value == 0 || core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value /= core[source].field_a.value;
                        core[destination].field_a.value /= core[source].field_b.value;
                    }
                }
            }
        }
        Opcode::Mod => { // mods instruction specified by B field by number(s) specified by A field
            match instruction.modifier {
                Modifier::A => {
                    if core[source].field_a.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value %= core[source].field_a.value;
                    }
                },
                Modifier::B => {
                    if core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value %= core[source].field_b.value;
                    }
                },
                Modifier::AB => {
                    if core[source].field_a.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value %= core[source].field_a.value;
                    }
                },
                Modifier::BA => {
                    if core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value %= core[source].field_b.value;
                    }
                },
                Modifier::F | Modifier::I => {
                    if core[source].field_a.value == 0 || core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_a.value %= core[source].field_a.value;
                        core[destination].field_b.value %= core[source].field_b.value;
                    }
                },
                Modifier::X => {
                    if core[source].field_a.value == 0 || core[source].field_b.value == 0 { dead = true }
                    else {
                        core[destination].field_b.value %= core[source].field_a.value;
                        core[destination].field_a.value %= core[source].field_b.value;
                    }
                }
            }
        }
        Opcode::Jmp => { // jumps to address specified by A field
            process_queue[0].pointer = source;
            step = false;
        }
        Opcode::Jmz => { // jumps to address specified by A field if field(s) specified by B field equals 0
            match instruction.modifier {
                Modifier::A | Modifier::BA => {
                    if core[destination].field_a.value == 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::B | Modifier::AB => {
                    if core[destination].field_b.value == 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::X | Modifier::F | Modifier::I => {
                    if core[destination].field_a.value == 0 && core[destination].field_b.value == 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
            }
        }
        Opcode::Jmn => { // jumps to address specified by A field if field(s) specified by B field are not equal to 0
            match instruction.modifier {
                Modifier::A | Modifier::BA => {
                    if core[destination].field_a.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::B | Modifier::AB => {
                    if core[destination].field_b.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::X | Modifier::F | Modifier::I => {
                    if core[destination].field_a.value != 0 || core[destination].field_b.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
            }
        }
        Opcode::Djn => { // decrements field specified by B field, then JMZs
            match instruction.modifier {
                Modifier::A | Modifier::BA => {
                    decrement_mod(&mut core[destination].field_a.value, coresize);
                    if core[destination].field_a.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::B | Modifier::AB => {
                    decrement_mod(&mut core[destination].field_b.value, coresize);
                    if core[destination].field_b.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
                Modifier::X | Modifier::F | Modifier::I => {
                    decrement_mod(&mut core[destination].field_a.value, coresize);
                    decrement_mod(&mut core[destination].field_b.value, coresize);
                    if core[destination].field_a.value != 0 || core[destination].field_b.value != 0 {
                        process_queue[0].pointer = source;
                        step = false;
                    }
                },
            }
        }
        Opcode::Spl => {
            let new_process = Process {
                team: process.team,
                pointer: source
            };
            process_queue.push_front(new_process); // push_front is used because rotate_left will be run right after, effectively being a push_back with no rotation
            step = false; // if this was true, the new process would be stepped instead of the current one
            increment_mod(&mut process_queue[1].pointer, coresize);
        }
        Opcode::Seq | Opcode::Cmp => { // skips next instruction if instructions specified by A and B field are equal
            let mut skip: bool = false;

            match instruction.modifier {
                Modifier::A =>
                    if core[destination].field_a.value == core[source].field_a.value { skip = true; },
                Modifier::B =>
                    if core[destination].field_b.value == core[source].field_b.value { skip = true; },
                Modifier::AB =>
                    if core[destination].field_b.value == core[source].field_a.value { skip = true; },
                Modifier::BA =>
                    if core[destination].field_a.value == core[source].field_b.value { skip = true; },
                Modifier::F =>
                    if core[destination].field_a.value == core[source].field_a.value && core[destination].field_b.value == core[source].field_b.value { skip = true; },
                Modifier::X =>
                    if core[destination].field_b.value == core[source].field_a.value && core[destination].field_a.value == core[source].field_b.value { skip = true; },
                Modifier::I =>
                    if core[destination] == core[source] { skip = true; },
            }

            if skip {
                process_queue[0].pointer += 2;
                step = false;
            }
        }
        Opcode::Sne => { // skips next instruction if instructions specified by A and B field are not equal
            let mut skip: bool = false;

            match instruction.modifier {
                Modifier::A =>
                    if core[destination].field_a.value != core[source].field_a.value { skip = true; },
                Modifier::B =>
                    if core[destination].field_b.value != core[source].field_b.value { skip = true; },
                Modifier::AB =>
                    if core[destination].field_b.value != core[source].field_a.value { skip = true; },
                Modifier::BA =>
                    if core[destination].field_a.value != core[source].field_b.value { skip = true; },
                Modifier::F =>
                    if core[destination].field_a.value != core[source].field_a.value || core[destination].field_b.value != core[source].field_b.value { skip = true; },
                Modifier::X =>
                    if core[destination].field_b.value != core[source].field_a.value || core[destination].field_a.value != core[source].field_b.value { skip = true; },
                Modifier::I =>
                    if core[destination] != core[source] { skip = true; },
            }

            if skip {
                process_queue[0].pointer += 2;
                step = false;
            }
        }
        Opcode::Slt => { // skips next instruction if instructions specified by A is less than by B
            let mut skip: bool = false;

            match instruction.modifier {
                Modifier::A =>
                    if core[destination].field_a.value >= core[source].field_a.value { skip = true; },
                Modifier::B =>
                    if core[destination].field_b.value >= core[source].field_b.value { skip = true; },
                Modifier::AB =>
                    if core[destination].field_b.value >= core[source].field_a.value { skip = true; },
                Modifier::BA =>
                    if core[destination].field_a.value >= core[source].field_b.value { skip = true; },
                Modifier::F | Modifier::I =>
                    if core[destination].field_a.value >= core[source].field_a.value && core[destination].field_b.value >= core[source].field_b.value { skip = true; },
                Modifier::X =>
                    if core[destination].field_b.value >= core[source].field_a.value && core[destination].field_a.value >= core[source].field_b.value { skip = true; },
            }

            if skip {
                process_queue[0].pointer += 2;
                step = false;
            }
        }
        // Opcode::Ldp => { // excuse me corewa.rs??
        //
        // }
        // Opcode::Sdp => {
        //
        // }
        Opcode::Nop => { }
    }

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
        if step { process_queue[0].pointer += 1 };
        process_queue[0].pointer %= coresize;
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
        println!("A process from team {turn} has died!");
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

fn start_play_thread(old_app: &EmarsApp) {
    let mut app = EmarsApp {
        core: old_app.core.clone(),
        coresize: old_app.coresize,
        default_instruction: old_app.default_instruction,
        core_view_size: old_app.core_view_size,
        teams_process_queues: old_app.teams_process_queues.clone(),
        turn: old_app.turn,
        playing: old_app.playing,
        play_delay: old_app.play_delay,
        last_step: old_app.last_step,
        state_sender: old_app.state_sender.clone(),
        state_receiver: channel::<(Vec<Instruction>, Vec<VecDeque<Process>>)>().1,
        play_step_count: old_app.play_step_count,
        play_step_limit: old_app.play_step_limit,
    };
    spawn(move || {
        loop {
            if !app.process_playing() { break; }
            sleep(Duration::from_millis(10))
        }
    });
}

impl EmarsApp {
    fn process_playing(&mut self) -> bool {
        if self.last_step.elapsed().as_millis() as usize > self.play_delay {
            let mut dead: bool = false;
            for _ in 0..(self.last_step.elapsed().as_millis() as usize / self.play_delay) {
                if self.play_step_count >= self.play_step_limit { return false; }
                full_step(&mut self.core, self.coresize, &mut self.teams_process_queues, &mut self.turn);
                self.play_step_count += 1;
                if self.teams_process_queues.len() <= 1 { dead = true; break; }
            }
            self.last_step = Instant::now();
            match self.state_sender.send((self.core.clone(), self.teams_process_queues.clone())) {
                Ok(_) => {},
                Err(_) => return false,
            }
            if dead { return false; }
        }
        return true;
    }

    pub fn press_play(&mut self) {
        if !self.playing {
            self.playing = true;
            self.last_step = Instant::now();
            start_play_thread(&self);
        } else {
            self.playing = false;
            (self.state_sender, self.state_receiver) = channel::<(Vec<Instruction>, Vec<VecDeque<Process>>)>();
        }
    }
}
