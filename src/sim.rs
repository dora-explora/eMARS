use corewars_core::load_file::{Opcode, Field, Instruction};
use corewars_parser as parser;
use std::fs::read_to_string;

#[allow(unused_mut)]

fn print_core(core: Vec<Instruction>) {
    for instruction in core {
        println!("{}", instruction.to_string());
    }
}

pub fn init(warrior_a_path: &str, warrior_b_path: &str, coresize: isize) {
    let warrior_a_file_string= read_to_string(warrior_a_path).expect("");
    let warrior_a = match corewars_parser::parse(warrior_a_file_string.as_str()) { // this one's yoinked straight from the source
        parser::Result::Ok(warrior, warnings) => { 
            println!("{:?}", &warnings);
            Ok(warrior)
        }
        parser::Result::Err(warrior, warnings) => {
            println!("{:?}", &warnings);
            Err(warrior)
        }
    }.unwrap();

    let warrior_b_file_string= read_to_string(warrior_b_path).expect("");
    let warrior_b = match corewars_parser::parse(warrior_b_file_string.as_str()) { // this one's yoinked straight from the source
        parser::Result::Ok(warrior, warnings) => { 
            println!("{:?}", &warnings);
            Ok(warrior)
        }
        parser::Result::Err(warrior, warnings) => {
            println!("{:?}", &warnings);
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

    let warrior_b_offset = 4; // arbitrary for now
    let warrior_b_origin: isize = match warrior_b.program.origin {Some(n) => n as isize, None => 0} - warrior_b_offset; // why is it minus? no clue but it works sooooo
    let warrior_b_instructions = warrior_b.program.instructions;
    for i in 0..warrior_b_instructions.len() {
        signed_index = i as isize - warrior_b_origin;
        while signed_index.is_negative() { signed_index += coresize; }
        index = (signed_index % coresize) as usize;
        core[index] = warrior_b_instructions[i].clone();
    }

    print_core(core);

}