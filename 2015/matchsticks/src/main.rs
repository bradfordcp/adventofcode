use std::fs;

use matchsticks::*;

fn main() {
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .expect("Error reading input.")
        .trim()
        .split("\n")
        .map(|line| line.to_owned())
        .collect();
    let difference = lines.clone().into_iter().fold(0_usize, |acc, ele| {
        let code = code_count(ele.as_str());
        let mem = char_count(ele.as_str());

        acc + (code - mem)
    });

    println!("Puzzle 1:");
    println!("Code vs String Difference: {difference}\n");

    let original_code_count = lines.clone().into_iter().fold(0_usize, |acc, ele| {
        let code = code_count(ele.as_str());

        acc + code
    });

    let encoded: Vec<String> = lines
        .clone()
        .into_iter()
        .map(|ele| encode(ele.as_str()))
        .collect();
    let code_count = encoded.clone().into_iter().fold(0_usize, |acc, ele| {
        let code = code_count(ele.as_str());

        acc + code
    });
    let difference = code_count - original_code_count;

    println!("Puzzle 2:");
    println!("Encoded vs Original: {difference}\n");
}
