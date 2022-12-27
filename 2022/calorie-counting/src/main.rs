use adventofcode;
use calorie_counting::*;

fn main() {
    let args = adventofcode::parse_args();
    let elves = adventofcode::read_input(&args.input, parse_elves);

    let max_calories = max_calories(&elves);

    println!(
        "The elf with the most calories is {}. They are carrying {} calories.",
        max_calories.0 + 1,
        max_calories.1
    );

    let top_three = top_three_calories(&elves);
    let top_three_total = top_three.0 .1 + top_three.1 .1 + top_three.2 .1;

    println!(
        "The top three elves by calorie count are: {:?}. Combined they are carrying {} calories.",
        top_three, top_three_total
    )
}
