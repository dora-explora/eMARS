#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{AddressMode, Modifier, Field as OtherField, Instruction as OtherInstruction, Opcode, Value::Literal};
use corewars_parser as parser;
use egui::Label;
use std::fs::read_to_string;
use rand::Rng;

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
    let opcode = old_instruction.opcode;
    let modifier = old_instruction.modifier;
    let field_a_address_mode = old_instruction.field_a.address_mode;
    let field_b_address_mode = old_instruction.field_b.address_mode;
    let field_a_value = match old_instruction.field_a.value {
        Literal(n) => negative_mod(n as isize, coresize),
        _ => panic!("corewars_core Value::Label found while translating field A to crate::sim::Field")
    };
    let field_b_value = match old_instruction.field_b.value {
        Literal(n) => negative_mod(n as isize, coresize),
        _ => panic!("corewars_core Value::Label found while translating field B to crate::sim::Field")
    };

    return Instruction {
        opcode,
        modifier,
        field_a: Field {
            address_mode: field_a_address_mode,
            value: field_a_value,
        },
        field_b: Field {
            address_mode: field_b_address_mode,
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

pub fn init(warrior_a_path: String, warrior_b_path: String, coresize: usize, default_instruction: Instruction) -> (Vec<Instruction>, Vec<Process>) {
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

    let mut core = vec![default_instruction; coresize as usize];

    let warrior_a_origin: usize = match warrior_a.program.origin {Some(n) => n as usize, None => 0};
    let warrior_a_instructions = warrior_a.program.instructions;
    let mut signed_index: isize;
    let mut index: usize;
    for i in 0..warrior_a_instructions.len() {
        signed_index = i as isize - warrior_a_origin as isize;
        index = negative_mod(signed_index, coresize);
        core[index] = translate_instruction(warrior_a_instructions[i].clone(), coresize);
    }
    let process_a = Process { team: 0, pointer: warrior_a_origin as usize };

    let mut rng = rand::rng();
    let warrior_b_origin: usize = match warrior_b.program.origin {Some(n) => n as usize, None => 0}; 
    let warrior_b_lower_bound = (warrior_a_instructions.len() - warrior_a_origin + coresize/80) as usize;
    let warrior_b_upper_bound = (coresize - warrior_a_origin - coresize/80) as usize;
    let warrior_b_offset = rng.random_range(warrior_b_lower_bound..warrior_b_upper_bound) as isize;
    let warrior_b_instructions = warrior_b.program.instructions;
    for i in 0..warrior_b_instructions.len() {
        signed_index = i as isize - warrior_b_origin as isize + warrior_b_offset as isize;
        index = negative_mod(signed_index, coresize);
        core[index] = translate_instruction(warrior_b_instructions[i].clone(), coresize);
    }
    let process_b = Process { team: 1, pointer: (warrior_b_offset) as usize };

    return (core, vec![process_a, process_b]);
}

pub fn step(mut core: Vec<Instruction>, mut processes: Vec<Process>) -> (Vec<Instruction>, Vec<Process>) { // steps with the first process in the processes queue and returns the new state of the core and 
    
    let process = processes[0];
    let instruction = core[process.pointer].clone();
    let mut new_core = core.clone();

    // process predecrements for field a
    if instruction.field_a.address_mode == AddressMode::PreDecIndirectA {
        core[instruction.field_a.value].field_a.value -= 1;
    } else if instruction.field_a.address_mode == AddressMode::PreDecIndirectB {
        core[instruction.field_a.value].field_b.value -= 1;
    }

    // process predecrements for field b
    if instruction.field_b.address_mode == AddressMode::PreDecIndirectA {
        core[instruction.field_b.value].field_a.value -= 1;
    } else if instruction.field_b.address_mode == AddressMode::PreDecIndirectB {
        core[instruction.field_b.value].field_b.value -= 1;
    }

    // big if block for all the opcodes
    if instruction.opcode == Opcode::Dat { // kills the process
        processes.remove(0); // this assumes that the current process is at the front!!!
    } else if instruction.opcode == Opcode::Mov { // moves instruction/values specified by A field to instruction specified by B field
        let source_instruction_pointer: usize; // pointer to first instruction of MOV; the instruction to be copied (relative address)
        let dest_instruction_pointer: usize; // pointer to second instruction of MOV; the instruction to be copied to (relative address)

        // if instruction.field_a.address_mode == AddressMode::Immediate {
        //     source_instruction_pointer = 0;
        // } else if instruction.field_a.address_mode == AddressMode::Direct {
        //     source_instruction_pointer = instruction.field_a.value;
        // } else if instruction.field_a.address_mode == AddressMode::IndirectA || instruction.field_a.address_mode == AddressMode::PostIncIndirectA || instruction.field_a.address_mode == AddressMode::PreDecIndirectA {
        //     source_instruction_pointer = old_core[instruction.field_a.value + process.pointer].field_a.value + instruction.field_a.value;
        // } else if instruction.field_a.address_mode == AddressMode::IndirectB || instruction.field_a.address_mode == AddressMode::PostIncIndirectB || instruction.field_a.address_mode == AddressMode::PreDecIndirectB {
        //     source_instruction_pointer = old_core[instruction.field_a.value + process.pointer].field_b.value + instruction.field_a.value;
        // }

        match instruction.field_a.address_mode {
            AddressMode::Immediate => 
            source_instruction_pointer = 0,
            AddressMode::Direct => 
            source_instruction_pointer = instruction.field_a.value,
            AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA => 
            source_instruction_pointer = core[instruction.field_a.value + process.pointer].field_a.value + instruction.field_a.value,
            AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB => 
            source_instruction_pointer = core[instruction.field_a.value + process.pointer].field_b.value + instruction.field_a.value
        }

        // if instruction.field_b.address_mode == AddressMode::Immediate {
        //     dest_instruction_pointer = 0;
        // } else if instruction.field_b.address_mode == AddressMode::Direct {
        //     dest_instruction_pointer = instruction.field_b.value;
        // } else if instruction.field_b.address_mode == AddressMode::IndirectA || instruction.field_b.address_mode == AddressMode::PostIncIndirectA || instruction.field_b.address_mode == AddressMode::PreDecIndirectA {
        //     dest_instruction_pointer = old_core[instruction.field_b.value + process.pointer].field_a.value + instruction.field_b.value;
        // } else if instruction.field_b.address_mode == AddressMode::IndirectB || instruction.field_b.address_mode == AddressMode::PostIncIndirectB || instruction.field_b.address_mode == AddressMode::PreDecIndirectB {
        //     dest_instruction_pointer = old_core[instruction.field_b.value + process.pointer].field_b.value + instruction.field_b.value;
        // }

        match instruction.field_b.address_mode {
            AddressMode::Immediate => 
            dest_instruction_pointer = 0,
            AddressMode::Direct => 
            dest_instruction_pointer = instruction.field_b.value,
            AddressMode::IndirectA | AddressMode::PostIncIndirectA | AddressMode::PreDecIndirectA => 
            dest_instruction_pointer = core[instruction.field_b.value + process.pointer].field_a.value + instruction.field_b.value,
            AddressMode::IndirectB | AddressMode::PostIncIndirectB | AddressMode::PreDecIndirectB => 
            dest_instruction_pointer = core[instruction.field_b.value + process.pointer].field_b.value + instruction.field_b.value
        }

        new_core[dest_instruction_pointer + process.pointer] = core[source_instruction_pointer + process.pointer] 

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

    return (new_core, processes);
}