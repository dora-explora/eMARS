use corewars_parser as parser;
use std::fs::read_to_string;

fn main() {
    let file_string= read_to_string("warriors/imp.red").expect("");
    let core = match corewars_parser::parse(file_string.as_str()) { // this one's yoinked straight from the source
        parser::Result::Ok(warrior, warnings) => { 
            println!("{:?}", &warnings);
            Ok(warrior)
        }
        parser::Result::Err(warrior, warnings) => {
            println!("{:?}", &warnings);
            Err(warrior)
        }
    };

    println!("{:?}", core)
}