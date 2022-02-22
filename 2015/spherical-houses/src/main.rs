use std::fs;
use spherical_houses::*;

fn main() {
    let directions: Vec<Direction> = fs::read_to_string("input.txt").expect("Error reading input.")
    .trim()
    .chars()
    .map(|direction| {
        match direction {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Encountered unexpected direction: {direction}.")
        }
    }).collect();

    let mut santa = Santa::default();
    for direction in directions.clone() {
        santa.traverse(direction)
    }
    let houses_visited = santa.houses_visited();
    println!("Puzzle 1:");
    println!("Houses visited: {houses_visited}\n");

    let mut santa = Santa::default();
    let mut robos = Santa::default();
    let mut santas_turn = true;
    for direction in directions.clone() {
        if santas_turn {
            santa.traverse(direction);
        } else {
            robos.traverse(direction);
        }

        santas_turn = !santas_turn;
    }
    let mut visited = santa.visited.clone();
    visited.append(&mut robos.visited.clone());
    let combined_santa = Santa{
        visited
    };
    let houses_visited = combined_santa.houses_visited();

    println!("Puzzle 2:");
    println!("Houses visited: {houses_visited}\n");
}
