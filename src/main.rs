use std::fs::read_to_string;

enum OpCode {
    DAT,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    JMP,
    JMZ,
    JMN,
    DJN,
    SPL,
    SEQ,
    SNE,
    SLT,
    LDP,
    STP,
    NOP
}

enum Mode {
    HASHTAG,
    DOLLAR,
    ASTERISK,
    AT,
    LEFTBRACKET,
    LESSTHAN,
    RIGHTBRACKET,
    GREATERTHAN
}

enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I
}

struct Cell {
    opcode: OpCode,
    modifier: Modifier, 
    mode_a: Mode,
    field_a: u16,
    mode_b: Mode,
    field_b: u16
}

impl Cell {
    fn from_string(string: String) -> Self {
        let split: Vec<&str> = string.split(' ').collect();

        let instruction: String = split[0].to_string().to_uppercase();

        let mode_a: char;
        let field_a: u16;
        if !split[1].chars().nth(0).expect("what.").is_numeric() {
            mode_a = split[1].chars().nth(0).expect("how.");
            field_a = split[1].split_at(1).1.parse::<u16>().expect("thats not a number");
        } else {
            mode_a = '$';
            field_a = split[1].parse::<u16>().expect("thats not a number");
        }

        let mode_b: char;
        let field_b: u16;
        if !split[1].chars().nth(0).expect("what.").is_numeric() {
            mode_b = split[2].chars().nth(0).expect("how.");
            field_b = split[2].split_at(1).1.parse::<u16>().expect("thats not a number");
        } else {
            mode_b = '$';
            field_b = split[2].parse::<u16>().expect("thats not a number");
        }
        return Cell { opcode, modifier, mode_a, field_a, mode_b, field_b }
    }
}

struct Warrior {
    instructions: Vec<String>,
    offset: u16
}

fn compile(string: String) -> Warrior {
    let str_lines: Vec<&str> = string.split('\n').collect(); // this only exists cause split makes an iterator over &str
    let mut lines: Vec<String> = Vec::new();
    for line in str_lines { lines.push(line.to_string()); }

    let mut instructions: Vec<String> = Vec::new();
    let offset: u16 = 0;

    return Warrior {instructions, offset};
}

fn main() {
    let filepath: &str = "file"; 
    let string = read_to_string(filepath).expect("Could not open file: it either doesn't exist, or doesn't only contain text.");
    warrior = compile(string);

    let cell_string: String = "DAT #0 #0".to_string(); 
    let cell: Cell = Cell::from_string(cell_string);
    println!("cell instruction: {0}\ncell mode and field A: {1}, {2}\ncell mode and field B: {3}, {4}", cell.opcode, cell.mode_a, cell.field_a, cell.mode_b, cell.field_b)
}
