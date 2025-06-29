#![allow(unused_mut, unused_imports)]

use corewars_core::load_file::{Opcode, Field, Instruction};
use corewars_parser as parser;
use std::fs::read_to_string;
use rand::Rng;

pub fn init(warrior_a_path: String, warrior_b_path: String, coresize: isize) -> Vec<Instruction> {
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

    let mut rng = rand::rng();
    let warrior_b_origin: isize = match warrior_b.program.origin {Some(n) => n as isize, None => 0}; // why is it minus? no clue but it works sooooo
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

    return core;

}