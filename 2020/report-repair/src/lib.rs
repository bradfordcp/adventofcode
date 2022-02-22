#![feature(test)]

/// Generates the product of two integers which equal the provided target sum
pub fn pair_product(values: Vec<u32>, target: u32) -> Option<u32> {
    match find_pair(values, target) {
        Some((l, r)) => Some(l * r),
        None => None,
    }
}

/// Finds two values within a vector which sum to the provided input
fn find_pair(mut values: Vec<u32>, target: u32) -> Option<(u32, u32)> {
    values.sort();

    let left_start = 0;
    let left_end = values.len() / 2;

    let right_end = values.len();

    for lidx in left_start..left_end {
        let n1 = values.get(lidx).unwrap();
        let right_start = lidx + 1;

        for ridx in right_start..right_end {
            let n2 = values.get(ridx).unwrap();

            if n1 + n2 == target {
                return Some((n1.clone(), n2.clone()));
            }
        }
    }

    None
}

/// Calculates the product of three values within a vector which sum to the provided input
pub fn triple_product(values: Vec<u32>, target: u32) -> Option<u32> {
    match find_triple(values, target) {
        Some((l, m, r)) => Some(l * m * r),
        None => None,
    }
}

/// Finds three values within a vector which sum to the provided input
fn find_triple(mut values: Vec<u32>, target: u32) -> Option<(u32, u32, u32)> {
    values.sort();

    let left_start = 0;
    let left_end = values.len() / 2;

    let mid_end = values.len() - 1;
    let right_end = values.len();

    for lidx in left_start..left_end {
        let n1 = values.get(lidx).unwrap();
        let mid_start = lidx + 1;

        for midx in mid_start..mid_end {
            let n2 = values.get(midx).unwrap();
            let right_start = midx + 1;

            for ridx in right_start..right_end {
                let n3 = values.get(ridx).unwrap();

                if n1 + n2 + n3 == target {
                    return Some((n1.clone(), n2.clone(), n3.clone()));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    fn entries_from_file(path: &str) -> Vec<u32> {
        std::fs::read_to_string(path)
            .unwrap()
            .split("\n")
            .map(|line| line.parse::<u32>().unwrap())
            .collect()
    }

    #[test]
    fn test_sample_pair_product() {
        let entries = entries_from_file("test/sample");
        assert_eq!(514579, pair_product(entries, 2020).unwrap())
    }

    #[test]
    fn test_input_pair_product() {
        let entries = entries_from_file("test/input");
        assert_eq!(290784, pair_product(entries, 2020).unwrap());
    }

    #[test]
    fn test_find_pair() {
        let entries = entries_from_file("test/sample");
        let target = 2020;
        let pair = find_pair(entries, target);
        assert_eq!(pair, Some((299, 1721)))
    }

    #[test]
    fn test_sample_triple_product() {
        let entries = entries_from_file("test/sample");
        assert_eq!(241861950, triple_product(entries, 2020).unwrap())
    }

    #[test]
    fn test_input_triple_product() {
        let entries = entries_from_file("test/input");
        assert_eq!(177337980, triple_product(entries, 2020).unwrap());
    }

    #[test]
    fn test_find_triple() {
        let entries = entries_from_file("test/sample");
        let target = 2020;
        let triple = find_triple(entries, target);
        assert_eq!(triple, Some((366, 675, 979)))
    }

    #[bench]
    fn bench_small_pair_product(b: &mut Bencher) {
        let entries = entries_from_file("test/sample");

        b.iter(|| {
            let entries = entries.clone();
            pair_product(entries, 2020).unwrap();
        })
    }

    #[bench]
    fn bench_small_triple_product(b: &mut Bencher) {
        let entries = entries_from_file("test/sample");

        b.iter(|| {
            let entries = entries.clone();
            triple_product(entries, 2020).unwrap();
        })
    }

    #[bench]
    fn bench_large_pair_product(b: &mut Bencher) {
        let entries = entries_from_file("test/input");

        b.iter(|| {
            let entries = entries.clone();
            pair_product(entries, 2020).unwrap();
        })
    }

    #[bench]
    fn bench_large_triple_product(b: &mut Bencher) {
        let entries = entries_from_file("test/input");

        b.iter(|| {
            let entries = entries.clone();
            triple_product(entries, 2020).unwrap();
        })
    }
}
