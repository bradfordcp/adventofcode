use std::fs;
use not_quite_lisp::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Utter failure reading input");
    let instructions = parse(input.as_str());
    let floor = climb_stairs(&instructions);

    println!("Puzzle 1:");
    println!("Climbed to floor: {floor}\n");

    let step = steps_to_basement(&instructions);

    println!("Puzzle 2:");
    println!("First entered basement at step: {step}");
}
