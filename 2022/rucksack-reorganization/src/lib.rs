/// An elf's rucksack
#[derive(Clone, Debug)]
pub struct Rucksack {
    /// Left compartment
    left: u64,

    /// Right compartment
    right: u64
}

impl From<&str> for Rucksack {
    /// Converts a raw string from the puzzle input into a Rucksack object
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use rucksack_reorganization::*;
    /// let r: Rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp".into();
    /// ```
    fn from(value: &str) -> Self {
        let chars: Vec<char> = value.chars().collect();
        let (left_compartment, right_compartment) = chars.split_at(value.len() / 2);

        let left = left_compartment.iter().fold(0u64, |acc, ele| {
            let code = priority(&ele);
            acc | 1 << code
        });

        let right = right_compartment.iter().fold(0u64, |acc, ele| {
            let code = priority(&ele);
            acc | 1 << code
        });

        Rucksack { left: left, right: right }
    }
}

impl Rucksack {
    /// Provides the item that is present within both the left and right compartment
    ///
    /// # Examples
    pub fn shared(&self) -> u64 {
        self.left & self.right
    }

    pub fn combined(&self) -> u64 {
        self.left | self.right
    }
}

/// Calculates the priority of an item
///
/// # Examples
///
/// ```rust
/// # use rucksack_reorganization::*;
/// assert_eq!(priority(&'a'), 1);
/// assert_eq!(priority(&'z'), 26);
/// assert_eq!(priority(&'A'), 27);
/// assert_eq!(priority(&'Z'), 52);
/// ```
pub fn priority(item: &char) -> u8 {
    let code = *item as u8;

    if code > 64 && code < 91 {
        code - 38
    } else if code > 96 && code < 123{
        code - 96
    } else {
        panic!("Item type not prioritized: {}", item)
    }
}
