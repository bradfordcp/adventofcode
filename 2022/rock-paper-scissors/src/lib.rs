#[macro_use]
extern crate lazy_static;
use regex::Regex;

use std::str::Lines;

/// Represents possible plays or moves in the classic game "Rock, Paper, Scissors"
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Play {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Play {
    /// Decodes encrypted String values into Rock, Paper, or Scissors
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let a: Play = "A".into();
    /// assert_eq!(a, Play::Rock);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let b: Play = "B".into();
    /// assert_eq!(b, Play::Paper);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let c: Play = "C".into();
    /// assert_eq!(c, Play::Scissors);
    /// ```

    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let x: Play = "X".into();
    /// assert_eq!(x, Play::Rock);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let y: Play = "Y".into();
    /// assert_eq!(y, Play::Paper);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Play;
    /// let z: Play = "Z".into();
    /// assert_eq!(z, Play::Scissors);
    /// ```
    fn from(value: &str) -> Self {
        match value {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            "X" => Play::Rock,
            "Y" => Play::Paper,
            "Z" => Play::Scissors,
            o => panic!("Encountered unexpected value {}", o)
        }
    }
}

impl Play {
    /// Returns the score for a given play
    ///
    /// # Examples:
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// assert_eq!(Play::Rock.score(), 1);
    /// assert_eq!(Play::Paper.score(), 2);
    /// assert_eq!(Play::Scissors.score(), 3);
    /// ```
    pub fn score(&self) -> u64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3
        }
    }
}

/// Represents each of the three different outcomes possible within a game of Rock, Paper, Scissors.
#[derive(Debug, PartialEq)]
pub enum Outcome {
    Win,
    Lose,
    Draw
}

impl From<(&Play, &Play)> for Outcome {
    fn from(value: (&Play, &Play)) -> Self {
        let (opponent, player) = value;
        match opponent {
            Play::Rock => match player {
                Play::Rock => Outcome::Draw,
                Play::Paper => Outcome::Win,
                Play::Scissors => Outcome::Lose
            },
            Play::Paper => match player {
                Play::Rock => Outcome::Lose,
                Play::Paper => Outcome::Draw,
                Play::Scissors => Outcome::Win
            },
            Play::Scissors => match player {
                Play::Rock => Outcome::Win,
                Play::Paper => Outcome::Lose,
                Play::Scissors => Outcome::Draw
            }
        }
    }
}

impl Outcome {
    /// Returns the score for a given Outcome
    ///
    /// # Examples:
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// assert_eq!(Outcome::Lose.score(), 0);
    /// assert_eq!(Outcome::Draw.score(), 3);
    /// assert_eq!(Outcome::Win.score(), 6);
    /// ```
    pub fn score(&self) -> u64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }

    /// Calculates a Play given a desired Outcome and opponent Play
    ///
    /// # Examples:
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Win.counter_for(&Play::Rock);
    /// assert_eq!(Play::Paper, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Win.counter_for(&Play::Paper);
    /// assert_eq!(Play::Scissors, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Win.counter_for(&Play::Scissors);
    /// assert_eq!(Play::Rock, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Lose.counter_for(&Play::Rock);
    /// assert_eq!(Play::Scissors, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Lose.counter_for(&Play::Paper);
    /// assert_eq!(Play::Rock, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Lose.counter_for(&Play::Scissors);
    /// assert_eq!(Play::Paper, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Draw.counter_for(&Play::Rock);
    /// assert_eq!(Play::Rock, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Draw.counter_for(&Play::Paper);
    /// assert_eq!(Play::Paper, p);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let p = Outcome::Draw.counter_for(&Play::Scissors);
    /// assert_eq!(Play::Scissors, p);
    /// ```
    pub fn counter_for(&self, opponent: &Play) -> Play {
        match self {
            Outcome::Win => match opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock
            },
            Outcome::Lose => match opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper
            },
            Outcome::Draw => opponent.clone()
        }
    }
}

impl From<&str> for Outcome {
    /// Decodes encrypted String values into Rock, Paper, or Scissors
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rock_paper_scissors::Outcome;
    /// let x: Outcome = "X".into();
    /// assert_eq!(x, Outcome::Lose);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Outcome;
    /// let y: Outcome = "Y".into();
    /// assert_eq!(y, Outcome::Draw);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::Outcome;
    /// let z: Outcome = "Z".into();
    /// assert_eq!(z, Outcome::Win);
    /// ```
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            o => panic!("Unexpected outcome encrypted value {}", o)
        }
    }
}

/// Decrypted round definitions
#[derive(Debug, PartialEq)]
pub enum Round {
    /// Version 1: Each decrypted field is read as a play or move. The first represents your opponent with the second being yours.
    V1(Play, Play),
    /// Version 2: Encoded as the opponents play and desired outcome
    V2(Play, Outcome)
}

impl Round {
    /// Calculates the score for a given round. Where the total score is the sum of the player's play score and outcome score.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v1("A Y");
    /// assert_eq!(r.score(), 8);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v1("B X");
    /// assert_eq!(r.score(), 1);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v1("C Z");
    /// assert_eq!(r.score(), 6);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("A Y");
    /// assert_eq!(r.score(), 4);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("B X");
    /// assert_eq!(r.score(), 1);
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("C Z");
    /// assert_eq!(r.score(), 7);
    /// ```
    pub fn score(&self) -> u64 {
        match self {
            Round::V1(opponent, player) => {
                let outcome: Outcome = (opponent, player).into();
                player.score() + outcome.score()
            },
            Round::V2(opponent, outcome) => {
                let player: Play = outcome.counter_for(&opponent);
                player.score() + outcome.score()
            }
        }

    }

    /// Parses an encrypted round into a decoded Round struct
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rock_paper_scissors::{Round, Play};
    /// let r: Round = Round::from_v1("A Y");
    /// assert_eq!(r, Round::V1(Play::Rock, Play::Paper));
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::{Round, Play};
    /// let r: Round = Round::from_v1("B X");
    /// assert_eq!(r, Round::V1(Play::Paper, Play::Rock));
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::{Round, Play};
    /// let r: Round = Round::from_v1("C Z");
    /// assert_eq!(r, Round::V1(Play::Scissors, Play::Scissors));
    /// ```
    pub fn from_v1(value: &str) -> Self {
        lazy_static! {
            static ref ROUND_RE: Regex = Regex::new(r"^([A-C]) ([X-Z])$")
        .expect("Could not compile round regular expression");
        }

        if let Some(cap) = ROUND_RE.captures(value) {
            let opponent = cap.get(1).expect(format!("Could not find opponent play {}", value).as_str()).as_str().into();
            let player = cap.get(2).expect(format!("Could not find player play {}", value).as_str()).as_str().into();

            Round::V1(opponent, player)
        } else {
            panic!("Could not parse round \"{}\".", value)
        }
    }

    /// Parses an encrypted round into a decoded Round struct
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("A Y");
    /// assert_eq!(r, Round::V2(Play::Rock, Outcome::Draw));
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("B X");
    /// assert_eq!(r, Round::V2(Play::Paper, Outcome::Lose));
    /// ```
    ///
    /// ```rust
    /// # use rock_paper_scissors::*;
    /// let r: Round = Round::from_v2("C Z");
    /// assert_eq!(r, Round::V2(Play::Scissors, Outcome::Win));
    /// ```
    pub fn from_v2(value: &str) -> Self {
        lazy_static! {
            static ref ROUND_RE: Regex = Regex::new(r"^([A-C]) ([X-Z])$")
        .expect("Could not compile round regular expression");
        }

        if let Some(cap) = ROUND_RE.captures(value) {
            let opponent = cap.get(1).expect(format!("Could not find opponent play {}", value).as_str()).as_str().into();
            let outcome = cap.get(2).expect(format!("Could not find outcome {}", value).as_str()).as_str().into();

            Round::V2(opponent, outcome)
        } else {
            panic!("Could not parse round \"{}\".", value)
        }
    }
}

/// Parses an encrypted strategy guide into a vector of Round structs.
///
/// # Example:
///
/// ```rust
/// # use rock_paper_scissors::*;
/// let lines = "
/// A Y
/// B X
/// C Z
/// ".trim().lines();
/// let rounds = parse_rounds_v1(lines);
/// assert_eq!(vec![Round::V1(Play::Rock, Play::Paper), Round::V1(Play::Paper, Play::Rock), Round::V1(Play::Scissors, Play::Scissors)], rounds);
/// ```
pub fn parse_rounds_v1(lines: Lines) -> Vec<Round> {
    lines.map(|line| Round::from_v1(line)).collect()
}

/// Parses an encrypted strategy guide into a vector of Round structs.
///
/// # Example:
///
/// ```rust
/// # use rock_paper_scissors::*;
/// let lines = "
/// A Y
/// B X
/// C Z
/// ".trim().lines();
/// let rounds = parse_rounds_v2(lines);
/// assert_eq!(vec![Round::V2(Play::Rock, Outcome::Draw), Round::V2(Play::Paper, Outcome::Lose), Round::V2(Play::Scissors, Outcome::Win)], rounds);
/// ```
pub fn parse_rounds_v2(lines: Lines) -> Vec<Round> {
    lines.map(|line| Round::from_v2(line)).collect()
}
