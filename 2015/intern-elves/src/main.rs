use intern_elves::{is_nice, NiceProtocol};
use std::fs;

fn main() {
    let strings: Vec<String> = fs::read_to_string("input.txt")
        .expect("Error reading input.")
        .trim()
        .split("\n")
        .map(|str| str.to_string())
        .collect();

    let nice_count = strings
        .clone()
        .into_iter()
        .filter(|subject| is_nice(NiceProtocol::V1, subject.as_str()))
        .count();
    println!("Puzzle 1:");
    println!("Nice strings: {nice_count}\n");

    let nice_count = strings
        .clone()
        .into_iter()
        .filter(|subject| is_nice(NiceProtocol::V2, subject.as_str()))
        .count();
    println!("Puzzle 2:");
    println!("Nice strings: {nice_count}\n");
}
