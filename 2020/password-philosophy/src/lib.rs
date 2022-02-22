#![feature(test)]

use std::{fmt, fs, str::FromStr};

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

lazy_static! {
    static ref ENTRY_REGEX: Regex = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
}

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Could not parse entry".fmt(f)
    }
}

#[derive(Debug, Clone)]
struct SledPasswordEntry {
    minimum: usize,
    maximum: usize,
    token: char,
    password: String,
}

impl FromStr for SledPasswordEntry {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match ENTRY_REGEX.captures(line) {
            Some(cap) => Ok(SledPasswordEntry {
                minimum: cap[1].parse::<usize>().unwrap(),
                maximum: cap[2].parse::<usize>().unwrap(),
                token: cap[3].parse::<char>().unwrap(),
                password: cap[4].to_owned(),
            }),
            None => Err(ParseError {}),
        }
    }
}

#[derive(Debug, Clone)]
struct TobogganPasswordEntry {
    left: usize,
    right: usize,
    token: char,
    password: String,
}

impl FromStr for TobogganPasswordEntry {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match ENTRY_REGEX.captures(line) {
            Some(cap) => Ok(TobogganPasswordEntry {
                left: cap[1].parse::<usize>().unwrap(),
                right: cap[2].parse::<usize>().unwrap(),
                token: cap[3].parse::<char>().unwrap(),
                password: cap[4].to_owned(),
            }),
            None => Err(ParseError {}),
        }
    }
}

trait PasswordValidator {
    fn valid(&self) -> bool;
}

impl PasswordValidator for SledPasswordEntry {
    fn valid(self: &Self) -> bool {
        let char_count = self
            .password
            .chars()
            .filter(|c| c == &self.token)
            .collect::<Vec<char>>()
            .len();

        char_count >= self.minimum && char_count <= self.maximum
    }
}

impl PasswordValidator for TobogganPasswordEntry {
    fn valid(self: &Self) -> bool {
        let left_match = self.password.chars().nth(self.left - 1).unwrap() == self.token;
        let right_match = self.password.chars().nth(self.right - 1).unwrap() == self.token;

        (left_match || right_match) && !(left_match && right_match)
    }
}

#[allow(dead_code)]
fn read_entries<T>(path: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.parse::<T>().unwrap())
        .collect()
}

#[allow(dead_code)]
fn valid_passwords<T: PasswordValidator>(entries: Vec<T>) -> usize {
    entries
        .into_iter()
        .map(|entry| entry.valid())
        .filter(|valid| *valid)
        .collect::<Vec<bool>>()
        .len()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_read_sled_entries() {
        let entries = read_entries::<SledPasswordEntry>("test/sample");
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_read_toboggan_entries() {
        let entries = read_entries::<TobogganPasswordEntry>("test/sample");
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_sample_valid_sled_passwords() {
        let entries = read_entries::<SledPasswordEntry>("test/sample");
        assert_eq!(valid_passwords(entries), 2);
    }

    #[test]
    fn test_long_valid_sled_passwords() {
        let entries = read_entries::<SledPasswordEntry>("test/input");
        assert_eq!(valid_passwords(entries), 519);
    }

    #[test]
    fn test_sample_valid_toboggan_passwords() {
        let entries = read_entries::<TobogganPasswordEntry>("test/sample");
        assert_eq!(valid_passwords(entries), 1);
    }

    #[test]
    fn test_long_valid_toboggan_passwords() {
        let entries = read_entries::<TobogganPasswordEntry>("test/input");
        assert_eq!(valid_passwords(entries), 708);
    }

    #[bench]
    fn bench_small_sled_password_validation(b: &mut Bencher) {
        let entries = read_entries::<SledPasswordEntry>("test/sample");

        b.iter(|| {
            let entries = entries.clone();
            valid_passwords(entries);
        })
    }

    #[bench]
    fn bench_small_toboggan_password_validation(b: &mut Bencher) {
        let entries = read_entries::<TobogganPasswordEntry>("test/sample");

        b.iter(|| {
            let entries = entries.clone();
            valid_passwords(entries);
        })
    }

    #[bench]
    fn bench_large_sled_password_validation(b: &mut Bencher) {
        let entries = read_entries::<SledPasswordEntry>("test/input");

        b.iter(|| {
            let entries = entries.clone();
            valid_passwords(entries);
        })
    }

    #[bench]
    fn bench_large_toboggan_password_validation(b: &mut Bencher) {
        let entries = read_entries::<TobogganPasswordEntry>("test/input");

        b.iter(|| {
            let entries = entries.clone();
            valid_passwords(entries);
        })
    }
}
