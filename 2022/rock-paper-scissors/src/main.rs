use adventofcode;
use rock_paper_scissors::*;

fn main() {
    let args = adventofcode::parse_args();
    let rounds = adventofcode::read_input(&args.input, |lines| parse_rounds_v1(lines));
    let score = rounds.iter().map(|round| round.score()).fold(0, |sum, score| sum + score);
    println!("Part 1: Total score: {}", score);

    let rounds = adventofcode::read_input(&args.input, |lines| parse_rounds_v2(lines));
    let score = rounds.iter().map(|round| round.score()).fold(0, |sum, score| sum + score);
    println!("Part 2: Total score: {}", score);
}
