use fire_hazard::*;
use std::fs;

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string("input.txt")
        .expect("Error reading input.")
        .trim()
        .lines()
        .map(|instruction| Instruction::parse(instruction))
        .collect();

    let mut ld = LightDisplay::new(1000, 1000);
    ld.process_instructions(&instructions);

    println!("Puzzle 1:");
    println!("Lights lit: {}", ld.total_lit());

    let mut rd = DimmableLightDisplay::new(1000, 1000);
    rd.process_instructions(&instructions);

    println!("Puzzle 2:");
    println!("Total brightness: {}", rd.total_brightness());
}
