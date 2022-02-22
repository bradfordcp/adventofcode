const ROW_COUNT: u16 = 128;
const COL_COUNT: u16 = 8;

#[allow(dead_code)]
fn seat_id(seat_spec: &str) -> u16 {
    let (row_spec, col_spec) = seat_spec.split_at(7);
    println!("Seat Spec: {}, Row Spec: {}, Col Spec: {}", seat_spec, row_spec, col_spec);

    let row = row(row_spec);
    let col = col(col_spec);

    (row * COL_COUNT) + col
}

fn row(spec: &str) -> u16 {
    println!("Row: {}", spec);
    spec.chars().fold((0, ROW_COUNT - 1), |acc, ele| {
        println!("acc: {:?}, ele: {:?}", acc, ele);
        match ele {
            'F' => {
                (acc.0, (acc.0 + (acc.1 - acc.0) / 2))
            },
            'B' => {
                (acc.0 + ((acc.1 - acc.0) / 2) + 1, acc.1)
            },
            other => {
                panic!("Encountered unexpected character: {}", other);
            }
        }
    }).0
}

fn col(spec: &str) -> u16 {
    println!("Col: {}", spec);
    spec.chars().fold((0, COL_COUNT - 1), |acc, ele| {
        println!("acc: {:?}, ele: {:?}", acc, ele);
        match ele {
            'L' => {
                (acc.0, (acc.0 + (acc.1 - acc.0) / 2))
            },
            'R' => {
                (acc.0 + ((acc.1 - acc.0) / 2) + 1, acc.1)
            },
            other => {
                panic!("Encountered unexpected character: {}", other);
            }
        }
    }).0
}

#[allow(dead_code)]
fn missing(seats: Vec<u16>) -> u16 {
    seats.into_iter()
        .fold((0, 0, 0), |acc, ele| {
            println!("{:?}", acc);
            if acc.0 == 0 {
                (ele, 0, 0)
            } else if acc.1 == 0 {
                (acc.0, ele, 0)
            } else {
                let l = acc.0;
                let s = acc.1;

                if s != l + 1 {
                    (acc.1, ele, l + 1)
                } else {
                    (acc.1, ele, acc.2)
                }
            }
        }).2
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_row() {
        let spec = "FBFBBFF";
        assert_eq!(row(spec), 44);
    }

    #[test]
    fn test_col() {
        let spec = "RLR";
        assert_eq!(col(spec), 5);
    }

    #[test]
    fn test_sample() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_highest_sample() {
        let biggest_id = fs::read_to_string("test/sample").unwrap()
            .lines()
            .map(|spec| seat_id(spec))
            .max();
        
        assert!(biggest_id.is_some());
        assert_eq!(biggest_id.unwrap(), 820);
    }

    #[test]
    fn test_highest_input() {
        let biggest_id = fs::read_to_string("test/input").unwrap()
            .lines()
            .map(|spec| seat_id(spec))
            .max();
        
        assert!(biggest_id.is_some());
        assert_eq!(biggest_id.unwrap(), 861);
    }

    #[test]
    fn test_find_seat_input() {
        let mut ids: Vec<u16> = fs::read_to_string("test/input").unwrap()
            .lines()
            .map(|spec| seat_id(spec))
            .collect();
        ids.sort();
        
        assert_eq!(missing(ids), 633);
    }
}
