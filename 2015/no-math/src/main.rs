use std::fs;
use no_math::*;

fn main() {
    let presents: Vec<PresentBox> = fs::read_to_string("input.txt").expect("Error reading input.")
    .trim()
    .split("\n")
    .into_iter()
    .map(|entry| {
        let mut dimensions = entry.split("x");
        PresentBox{
            l: dimensions.next().expect("No dimension found").parse::<u64>().expect("Dimension not a u64"),
            w: dimensions.next().expect("No dimension found").parse::<u64>().expect("Dimension not a u64"),
            h: dimensions.next().expect("No dimension found").parse::<u64>().expect("Dimension not a u64")
        }
    }).collect();

    let paper = presents.clone().into_iter()
        .fold(0, |acc, pb| acc + pb.required_paper());
    println!("Puzzle 1:");
    println!("Required paper: {paper}\n");
    
    let ribbon = presents.clone().into_iter()
        .fold(0, |acc, present| acc + present.required_ribbon());
    println!("Puzzle 2:");
    println!("Required ribbon: {ribbon}\n");
}
