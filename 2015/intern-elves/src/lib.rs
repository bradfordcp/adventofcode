use std::collections::BTreeSet;

const VOWELS: &str = "aeiou";

pub enum NiceProtocol {
    V1,
    V2,
}

/// Counts the number of vowels present in the subject string.
fn vowel_count(subject: &str) -> usize {
    subject
        .chars()
        .filter(|c| VOWELS.contains(c.to_string().as_str()))
        .count()
}

/// Returns true if the subject has at least 3 vowels
fn has_at_least_3_vowels(subject: &str) -> bool {
    vowel_count(subject) >= 3
}

/// Returns true if there is any character that is repeated within a window
fn has_repeating_characters(subject: &str, window_size: usize) -> bool {
    if subject.len() < 1 + window_size {
        return false;
    }

    let mut iter = subject.chars();
    let mut window = vec![];

    loop {
        if window.len() == window_size {
            break;
        }

        window.push(iter.next().expect("Could not unwrap item into window"))
    }

    loop {
        match iter.next() {
            None => return false,
            Some(c) => {
                let l = window.first().expect("Missing first item in window");

                if l == &c {
                    return true;
                } else {
                    window.remove(0);
                    window.push(c);
                }
            }
        }
    }
}

/// Returns true if the subject contains the string "ab", "cd", "pq", or "xy".
fn has_forbidden_strings(subject: &str) -> bool {
    let forbidden = vec!["ab", "cd", "pq", "xy"];
    forbidden
        .into_iter()
        .map(|pat| subject.contains(pat))
        .fold(false, |acc, cond| acc || cond)
}

/// Returns true if the string has any substring of length 2 repeated throughout its string
fn has_repeating_pair(subject: &str) -> bool {
    if subject.len() < 4 {
        return false;
    }

    // Generate all pairs
    let mut iter = subject.chars();
    let mut last = iter
        .next()
        .expect("Could not obtain character from subject");
    let mut pairs = vec![];

    loop {
        match iter.next() {
            None => break,
            Some(c) => {
                let pair = format!("{last}{c}");

                match pairs.last() {
                    None => pairs.push(pair),
                    Some(last) => {
                        if last != &pair {
                            pairs.push(pair);
                        }
                    }
                }

                last = c;
            }
        }
    }

    let mut set: BTreeSet<String> = BTreeSet::new();
    for e in pairs {
        if set.contains(&e) {
            return true;
        } else {
            set.insert(e);
        }
    }

    false
}

pub fn is_nice(protocol: NiceProtocol, subject: &str) -> bool {
    match protocol {
        NiceProtocol::V1 => {
            has_at_least_3_vowels(subject)
                && has_repeating_characters(subject, 1)
                && !has_forbidden_strings(subject)
        }
        NiceProtocol::V2 => has_repeating_pair(subject) && has_repeating_characters(subject, 2),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vowel_count() {
        assert_eq!(vowel_count("foo"), 2);
        assert_eq!(vowel_count("qs"), 0);
    }

    #[test]
    fn test_has_at_least_3_vowels() {
        assert!(!has_at_least_3_vowels(""));
        assert!(!has_at_least_3_vowels("abc"));
        assert!(has_at_least_3_vowels("aaa"));
    }

    #[test]
    fn test_has_repeating_characters() {
        assert!(has_repeating_characters("aa", 1));
        assert!(has_repeating_characters("abb", 1));
        assert!(has_repeating_characters("aab", 1));
        assert!(has_repeating_characters("abccdef", 1));
        assert!(!has_repeating_characters("", 1));
        assert!(!has_repeating_characters("a", 1));

        assert!(has_repeating_characters("xyx", 2));
        assert!(has_repeating_characters("abcdefeghi", 2));
        assert!(has_repeating_characters("aaa", 2));
        assert!(!has_repeating_characters("abc", 2));
    }

    #[test]
    fn test_has_forbidden_strings() {
        assert!(has_forbidden_strings("ab"));
        assert!(has_forbidden_strings("cd"));
        assert!(has_forbidden_strings("pq"));
        assert!(has_forbidden_strings("xy"));
        assert!(!has_forbidden_strings("ef"));
    }

    #[test]
    fn test_is_nice_v1() {
        assert!(is_nice(NiceProtocol::V1, "ugknbfddgicrmopn"));
        assert!(is_nice(NiceProtocol::V1, "aaa"));
        assert!(!is_nice(NiceProtocol::V1, "jchzalrnumimnmhp"));
        assert!(!is_nice(NiceProtocol::V1, "haegwjzuvuyypxyu"));
        assert!(!is_nice(NiceProtocol::V1, "dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_has_repeating_pair() {
        assert!(has_repeating_pair("xyxy"));
        assert!(has_repeating_pair("aabcdefgaa"));
        assert!(!has_repeating_pair("aaa"));
        assert!(!has_repeating_pair("aaab"));
        assert!(!has_repeating_pair("baaab"));
    }

    #[test]
    fn test_is_nice_v2() {
        assert!(is_nice(NiceProtocol::V2, "qjhvhtzxzqqjkmpb"));
        assert!(is_nice(NiceProtocol::V2, "xxyxx"));
        assert!(!is_nice(NiceProtocol::V2, "uurcxstgmygtbstg"));
        assert!(!is_nice(NiceProtocol::V2, "ieodomkazucvgmuy"));
    }
}
