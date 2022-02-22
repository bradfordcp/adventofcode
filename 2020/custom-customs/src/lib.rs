use std::collections::HashSet;

#[allow(dead_code)]
fn parse_group_any(group: &str) -> HashSet<char> {
    group.lines()
        .map(|line| parse_response(line))
        .fold(HashSet::new(), |acc, ele| {
            acc.union(&ele).into_iter()
                .map(|c| c.clone())
                .collect()
        })
}

#[allow(dead_code)]
fn parse_group_every(group: &str) -> HashSet<char> {
    group.lines()
        .map(|line| {
            parse_response(line)
        })
        .fold(None, |acc: Option<HashSet<char>>, ele: HashSet<char>| {
            if acc.is_some() {
                Some(acc.unwrap().intersection(&ele).into_iter()
                    .map(|c| c.clone())
                    .collect::<HashSet<char>>())
            } else {
                Some(ele)
            }
        }).unwrap()
}

fn parse_response(line: &str) -> HashSet<char> {
    line.chars().fold(HashSet::new(), |mut acc, ele| {
        acc.insert(ele);
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_parse_response() {
        let line = "abcx";
        let chars = parse_response(line);

        assert_eq!(chars.len(), 4);
    }

    #[test]
    fn test_parse_group_any() {
        let group = "abcx\nabcy\nabcz";
        let uniq = parse_group_any(group);

        assert_eq!(uniq.len(), 6);
    }

    #[test]
    fn test_parse_group_every() {
        let group = "abcx\nabcy\nabcz";
        let uniq = parse_group_every(group);

        assert_eq!(uniq.len(), 3);
    }

    #[test]
    fn test_sum_sample_any() {
        let sum= fs::read_to_string("test/sample").unwrap()
            .split("\n\n")
            .map(|group| parse_group_any(group))
            .fold(0, |acc, ele| acc + ele.len());
        assert_eq!(sum, 11);
    }

    #[test]
    fn test_sum_input_any() {
        let sum= fs::read_to_string("test/input").unwrap()
            .split("\n\n")
            .map(|group| parse_group_any(group))
            .fold(0, |acc, ele| acc + ele.len());
        assert_eq!(sum, 6291);
    }

    #[test]
    fn test_sum_sample_every() {
        let sum= fs::read_to_string("test/sample").unwrap()
            .split("\n\n")
            .map(|group| parse_group_every(group))
            .fold(0, |acc, ele| {
                acc + ele.len()
            });
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_sum_input_every() {
        let sum= fs::read_to_string("test/input").unwrap()
            .split("\n\n")
            .map(|group| parse_group_every(group))
            .fold(0, |acc, ele| acc + ele.len());
        assert_eq!(sum, 3052);
    }
}
