#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{AddressMode, Modifier, Field as OtherField, Instruction as OtherInstruction, Opcode, Value::Literal};
use corewars_parser as parser;
use egui::Label;
use std::fs::read_to_string;
use rand::Rng;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Process {
    pub(crate) team: u8,
    pub(crate) pointer: usize
}

#[derive(Clone, Copy, PartialEq)]
pub struct Field {
    pub(crate) address_mode: AddressMode,
    pub(crate) value: i32
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}{}", self.address_mode, self.value))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Instruction {
    pub(crate) opcode: Opcode,
    pub(crate) modifier: Modifier,
    pub(crate) field_a: Field,
    pub(crate) field_b: Field
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            // Example output:
            // MOV.AB  $-100,  $1
            // |----->||----->|
            "{op:<8}{a:<8}{b}",
            op = format!("{}.{}", self.opcode, self.modifier),
            a = format!("{},", self.field_a),
            b = self.field_b,
        ))
    }
}

fn translate_instruction(old_instruction: OtherInstruction) -> Instruction {
    let opcode = old_instruction.opcode;
    let modifier = old_instruction.modifier;
    let field_a_address_mode = old_instruction.field_a.address_mode;
    let field_b_address_mode = old_instruction.field_b.address_mode;
    let field_a_value = match old_instruction.field_a.value {
        Literal(n) => n,
        _ => panic!("corewars_core Value::Label found while translating field A to crate::sim::Field")
    };
    let field_b_value = match old_instruction.field_b.value {
        Literal(n) => n,
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

pub fn init(warrior_a_path: String, warrior_b_path: String, coresize: isize, default_instruction: Instruction) -> (Vec<Instruction>, Vec<Process>) {
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

    let warrior_a_origin: isize = match warrior_a.program.origin {Some(n) => n as isize, None => 0};
    let warrior_a_instructions = warrior_a.program.instructions;
    let mut signed_index: isize;
    let mut index: usize;
    for i in 0..warrior_a_instructions.len() {
        signed_index = i as isize - warrior_a_origin;
        while signed_index.is_negative() { signed_index += coresize; }
        index = (signed_index % coresize) as usize;
        core[index] = translate_instruction(warrior_a_instructions[i].clone());
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
        core[index] = translate_instruction(warrior_b_instructions[i].clone());
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

                // process predecrements
                

                // big if block for all the opcodes!!
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