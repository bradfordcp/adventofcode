use std::fs;
use fire_hazard::*;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").expect("Could not build regex");

    let instructions: Vec<Instruction> = fs::read_to_string("input.txt").expect("Error reading input.")
    .trim()
    .lines()
    .map(|instruction| {
        let cap = re.captures(instruction).expect("Could not find any captures");
        let tl = (
            cap.get(2).expect("TL X coordinate not found").as_str().parse::<usize>().expect("Could not parse coordinate"),
            cap.get(3).expect("TL Y coordinate not found").as_str().parse::<usize>().expect("Could not parse coordinate")
        );
        let br = (
            cap.get(4).expect("BR X coordinate not found").as_str().parse::<usize>().expect("Could not parse coordinate"),
            cap.get(5).expect("BR Y coordinate not found").as_str().parse::<usize>().expect("Could not parse coordinate")
        );

        match cap.get(1).expect("Could not match instruction").as_str() {
            "turn on" => Instruction::TurnOn(tl, br),
            "turn off" => Instruction::TurnOff(tl, br),
            "toggle" => Instruction::Toggle(tl, br),
            inst => panic!("Unexpected instruction encountered {}", inst)
        }
    }).collect();

    let mut ld = LightDisplay::new(1000, 1000);
    ld.process_instructions(&instructions);

    println!("Puzzle 1:");
    println!("Lights lit: {}", ld.total_lit());

    let mut rd = DimmableLightDisplay::new(1000, 1000);
    rd.process_instructions(&instructions);

    println!("Puzzle 2:");
    println!("Total brightness: {}", rd.total_brightness());
}
