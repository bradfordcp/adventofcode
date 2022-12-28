use adventofcode;
use rucksack_reorganization::*;

pub fn main() {
    let args = adventofcode::parse_args();
    let rucksacks: Vec<Rucksack> = adventofcode::read_input(&args.input, |lines| lines.map(|line| line.into()).collect());

    let priority_sum = rucksacks.iter().fold(0 as u64, |acc, ele| {
        let shared = ele.shared();

        for i in 1..57 {
            if shared & (1 << i) != 0 {
                return acc + i
            }
        }

        panic!("Could not find item type for set: {}", shared)
    });
    println!("Summed priority: {}", priority_sum);

    let groups: Vec<Vec<&Rucksack>> = rucksacks.iter().fold((vec![], vec![]), |acc, ele| {
        let (group, groups) = acc;

        let group = [group, vec![ele]].concat();

        if group.len() == 3 {
            (vec![], [groups, vec![group]].concat())
        } else {
            (group, groups)
        }
    }).1;

    let priority_sum = groups.iter().map(|group| {
        group[0].combined() & group[1].combined() & group[2].combined()
    }).fold(0u64, |acc, ele| {
        for i in 1..57 {
            if ele & (1 << i) != 0 {
                return acc + i
            }
        }

        panic!("Could not find item type for set: {}", ele)
    });
    println!("Summed priority: {}", priority_sum);
}
