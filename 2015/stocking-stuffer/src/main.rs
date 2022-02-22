use stocking_stuffer::Miner;

fn main() {
    let miner = Miner::new("bgvyzdsv");
    let solution = miner.mine("00000");

    println!("Puzzle 1:");
    println!("Solution: {solution}\n");

    let solution = miner.mine("000000");
    println!("Puzzle 2:");
    println!("Solution: {solution}\n");
}
