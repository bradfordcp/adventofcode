use std::str::Lines;

/// Parses calorie totals for a group of elves given a list of text file lines. Each line is a single snack for a given elf with blank lines separating each elf. Each elf is given an entry in the resulting Vec with their total amount of calories stored at the value.
///
/// # Example:
///
/// ```rust
/// let input = "
/// 1000
/// 2000
/// 3000
///
/// 4000
///
/// 5000
/// 6000
///
/// 7000
/// 8000
/// 9000
///
/// 10000
/// ";
///
/// let elves = calorie_counting::parse_elves(input.lines());
/// assert_eq!(elves.len(), 5);
/// assert_eq!(elves, vec![6000, 4000, 11000, 24000, 10000]);
/// ```
pub fn parse_elves(lines: Lines) -> Vec<u64> {
    lines
        .fold((true, vec![]), |acc, ele| {
            let (new_record, elves) = acc;

            if ele.len() == 0 {
                (true, elves)
            } else {
                let value = ele.parse::<u64>().unwrap();

                if new_record {
                    (false, [elves, vec![value]].concat())
                } else {
                    if let Some((last, others)) = elves.split_last() {
                        let sum = last + value;
                        (false, [others, &[sum]].concat())
                    } else {
                        panic!("Could not split elves :(")
                    }
                }
            }
        })
        .1
}

/// Iterates over the provided vector of elves returning the one carrying the most calories.
///
/// # Example:
///
/// ```rust
/// let elves: Vec<u64> = vec![6000, 4000, 11000, 24000, 10000];
/// let max = calorie_counting::max_calories(&elves);
/// assert_eq!(max, (3, &24000));
/// ```
pub fn max_calories(elves: &Vec<u64>) -> (usize, &u64) {
    elves
        .iter()
        .enumerate()
        .map(|ele| ele)
        .reduce(|acc, ele| if ele.1 > acc.1 { ele } else { acc })
        .unwrap()
}

/// Identifies which elves provide the most calories out of the entire set
///
/// # Example:
///
/// ```rust
/// let elves: Vec<u64> = vec![6000, 4000, 11000, 24000, 10000];
/// let top_three = calorie_counting::top_three_calories(&elves);
/// assert_eq!(top_three, ((3, &24000), (2, &11000), (4, &10000)));
/// ```
pub fn top_three_calories(elves: &Vec<u64>) -> ((usize, &u64), (usize, &u64), (usize, &u64)) {
    let top_three = elves
        .iter()
        .enumerate()
        .fold((None, None, None), |acc, ele| match acc {
            (None, None, None) => (Some(ele), None, None),
            (Some(a), None, None) => {
                if ele.1 > a.1 {
                    (Some(ele), Some(a), None)
                } else {
                    (Some(a), Some(ele), None)
                }
            }
            (Some(a), Some(b), None) => {
                if ele.1 > a.1 {
                    (Some(ele), Some(a), Some(b))
                } else if ele.1 > b.1 {
                    (Some(a), Some(ele), Some(b))
                } else {
                    (Some(a), Some(b), Some(ele))
                }
            }
            (Some(a), Some(b), Some(c)) => {
                if ele.1 > a.1 {
                    (Some(ele), Some(a), Some(b))
                } else if ele.1 > b.1 {
                    (Some(a), Some(ele), Some(b))
                } else if ele.1 > c.1 {
                    (Some(a), Some(b), Some(ele))
                } else {
                    (Some(a), Some(b), Some(c))
                }
            }
            _ => panic!("Encountered unexpected accumulator {:?}", acc),
        });

    (
        top_three.0.unwrap(),
        top_three.1.unwrap(),
        top_three.2.unwrap(),
    )
}
