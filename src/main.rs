use std::fs::read_to_string;

#[derive(PartialEq)]
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

impl OpCode {
    fn from_str(str: &str) -> OpCode {
        return match str.to_uppercase().as_str() {
            "DAT" => OpCode::DAT,
            "MOV" => OpCode::MOV,
            "ADD" => OpCode::ADD,
            "SUB" => OpCode::SUB,
            "MUL" => OpCode::MUL,
            "DIV" => OpCode::DIV,
            "MOD" => OpCode::MOD,
            "JMP" => OpCode::JMP,
            "JMZ" => OpCode::JMZ,
            "JMN" => OpCode::JMN,
            "DJN" => OpCode::DJN,
            "SPL" => OpCode::SPL,
            "SEQ" => OpCode::SEQ,
            "CMP" => OpCode::SEQ,
            "SNE" => OpCode::SNE,
            "SLT" => OpCode::SLT,
            "LDP" => OpCode::LDP,
            "STP" => OpCode::STP,
            "NOP" => OpCode::NOP,
            _ => panic!("thats not an opcode")
        }
    }
}

#[derive(PartialEq)]
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

impl Mode {
    fn from_char(char: char) -> Mode {
        return match char {
            '#' => Mode::HASHTAG,
            '$' => Mode::DOLLAR,
            '*' => Mode::ASTERISK,
            '@' => Mode::AT,
            '{' => Mode::LEFTBRACKET,
            '<' => Mode::LESSTHAN,
            '}' => Mode::RIGHTBRACKET,
            '>' => Mode::GREATERTHAN,
            _ => panic!("thats not a mode")
        }
    }
}

#[derive(PartialEq)]
enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I
}

impl Modifier {
    fn from_str(string: &str) -> Modifier {
        return match string.to_uppercase().as_str() {
            "A" => Modifier::A,
            "B" => Modifier::B,
            "AB" => Modifier::AB,
            "BA" => Modifier::BA,
            "F" => Modifier::F,
            "X" => Modifier::X,
            "I" => Modifier::I,
            _ => panic!("thats not a modifier")
        }
    }
}

#[derive(PartialEq)]
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

        let mode_a: Mode;
        let field_a: u16;
        if !split[1].chars().nth(0).expect("what.").is_numeric() {
            mode_a = Mode::from_char(split[1].chars().nth(0).expect("how."));
            field_a = split[1].split_at(1).1.parse::<u16>().expect("thats not a number");
        } else {
            mode_a = Mode::from_char('$');
            field_a = split[1].parse::<u16>().expect("thats not a number");
        }

        let mode_b: Mode;
        let field_b: u16;
        if !split[2].chars().nth(0).expect("what.").is_numeric() {
            mode_b = Mode::from_char(split[2].chars().nth(0).expect("how."));
            field_b = split[2].split_at(1).1.parse::<u16>().expect("thats not a number");
        } else {
            mode_b = Mode::from_char('$');
            field_b = split[2].parse::<u16>().expect("thats not a number");
        }

        let opcode: OpCode;
        let modifier: Modifier;
        if split[0].len() > 3 {
            let splitter: Vec<&str> = split[0].split('.').collect();
            opcode = OpCode::from_str(splitter[0]);
            modifier = Modifier::from_str(splitter[1]);
        } else { 
            opcode = OpCode::from_str(split[0]);
            if opcode == OpCode::DAT || opcode == OpCode::NOP {
                modifier = Modifier::F;
            } else if opcode == OpCode::JMP || opcode == OpCode::JMN || opcode == OpCode::JMZ || opcode == OpCode::DJN || opcode == OpCode::SPL {
                modifier = Modifier::B;
            } else if opcode == OpCode::MOV || opcode == OpCode::SEQ || opcode == OpCode::SNE {
                if mode_a == Mode::HASHTAG { modifier = Modifier::AB; }
                else if mode_b == Mode::HASHTAG { modifier = Modifier::B; }
                else { modifier = Modifier::I; }
            } else if opcode == OpCode::ADD || opcode == OpCode::SUB || opcode == OpCode::MUL || opcode == OpCode::DIV || opcode == OpCode::MOD {
                if mode_a == Mode::HASHTAG { modifier = Modifier::AB; }
                else if mode_b == Mode::HASHTAG { modifier = Modifier::B; }
                else { modifier = Modifier::F; }
            } else {
                if mode_a == Mode::HASHTAG { modifier = Modifier::AB; }
                else { modifier = Modifier::B; }
            }
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
    // let filepath: &str = "file"; 
    // let string = read_to_string(filepath).expect("Could not open file: it either doesn't exist, or doesn't only contain text.");
    let cell = Cell::from_string("CMP 1 #2".to_string());
    let opcode = OpCode::SEQ;
    let modifier = Modifier::B;
    let mode_a = Mode::DOLLAR;
    let field_a: u16 = 1;
    let mode_b = Mode::HASHTAG;
    let field_b: u16 = 2;
    println!("{}", cell == Cell { opcode, modifier, mode_a, field_a, mode_b, field_b });
}
