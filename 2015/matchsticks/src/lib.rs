pub fn code_count(input: &str) -> usize {
    input.chars().count()
}

pub fn char_count(input: &str) -> usize {
    let ESCAPE_PREFIX = "\\";
    let ESCAPE_HEX_PREFIX = "\\x";

    input
        .chars()
        .into_iter()
        .fold((0, None), |acc: (usize, Option<String>), ele| match acc {
            (sum, None) => match ele {
                '\\' => (sum, Some(format!("{}", ele))),
                _ => (sum + 1, None),
            },
            (sum, Some(prefix)) => {
                if prefix.starts_with(ESCAPE_HEX_PREFIX) {
                    if prefix.len() == 3 {
                        (sum + 1, None)
                    } else {
                        (sum, Some(format!("{}{}", prefix, ele)))
                    }
                } else if prefix.starts_with(ESCAPE_PREFIX) {
                    match ele {
                        '\\' => (sum + 1, None),
                        '"' => (sum + 1, None),
                        'x' => (sum, Some(format!("{}{}", prefix, ele))),
                        _ => panic!("Invalid escape sequence. {}{}", prefix, ele),
                    }
                } else {
                    panic!("Invalid escape sequence. {}{}", prefix, ele)
                }
            }
        })
        .0
        - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_code_count() {
        let input = "\"\"";

        assert_eq!(code_count(input), 2);
        assert_eq!(char_count(input), 0)
    }

    #[test]
    fn test_no_escapes() {
        let input = "\"abc\"";

        assert_eq!(code_count(input), 5);
        assert_eq!(char_count(input), 3);
    }

    #[test]
    fn test_escape_quote() {
        let input = "\"aaa\\\"aaa\"";

        assert_eq!(code_count(input), 10);
        assert_eq!(char_count(input), 7)
    }

    #[test]
    fn test_escape_hex() {
        let input = "\"\\x27\"";

        assert_eq!(code_count(input), 6);
        assert_eq!(char_count(input), 1);
    }
}
