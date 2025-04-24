struct Cell {
    instruction: String,
    mode_a: char,
    field_a: u16,
    mode_b: char,
    field_b: u16
}

impl Cell {
    fn from_string(string: String) -> Self {
        let split: Vec<&str> = string.split(' ').collect();

        let instruction: String = split[0].to_string();

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
        return Cell { instruction, mode_a, field_a, mode_b, field_b }
    }
}

fn main() {
    let cell_string: String = "DAT #0 #0".to_string(); 
    let cell: Cell = Cell::from_string(cell_string);
    println!("cell instruction: {0}\ncell mode and field A: {1}, {2}\ncell mode and field B: {3}, {4}", cell.instruction, cell.mode_a, cell.field_a, cell.mode_b, cell.field_b)
}
