use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Puzzle input file
    #[arg(short, long)]
    pub input: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn read_input<T>(path: &String, f: fn(std::str::Lines) -> Vec<T>) -> Vec<T> {
    let input = fs::read_to_string(path).expect(format!("Could not read file {}", path).as_str());
    let input = input.trim().lines();

    f(input)
}
