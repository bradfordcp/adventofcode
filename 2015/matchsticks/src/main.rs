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
}
