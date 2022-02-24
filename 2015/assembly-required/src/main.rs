use assembly_required::*;
use assembly_required::signal::*;
use std::fs;

fn main() {
    let schematic: Vec<Signal> = fs::read_to_string("input.txt")
        .expect("Error reading input.")
        .trim()
        .lines()
        .map(|signal| Signal::parse(signal))
        .collect();
    let circuit = SimpleCircuit::new(&schematic);

    println!("Puzzle 1:");
    println!("a value: {}\n", circuit.get("a").expect("Could not find value for wire \"a\"."));

    let mut schematic: Vec<Signal> = schematic
        .iter()
        .filter(|signal| match signal {
            Signal::VALUE(Component::ID(id), _) => {
                !id.eq(&"b".to_string())
            },
            _ => true
        })
        .map(|signal| signal.to_owned())
        .collect();
    schematic.push(Signal::parse("3167 -> b"));
    let circuit = SimpleCircuit::new(&schematic);

    println!("Puzzle 2:");
    println!("a value: {}", circuit.get("a").expect("Could not find value for wire \"a\"."));
}
