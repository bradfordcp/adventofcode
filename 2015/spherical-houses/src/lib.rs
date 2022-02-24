use std::collections::BTreeSet;

pub struct Santa {
    pub visited: Vec<(i64, i64)>,
}

#[derive(Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Default for Santa {
    fn default() -> Self {
        Santa {
            visited: vec![(0, 0)],
        }
    }
}

impl Santa {
    pub fn traverse(&mut self, direction: Direction) {
        let last = self.visited.last().expect("Error fetching last location");

        match direction {
            Direction::North => self.visited.push((last.0, last.1 + 1)),
            Direction::South => self.visited.push((last.0, last.1 - 1)),
            Direction::East => self.visited.push((last.0 + 1, last.1)),
            Direction::West => self.visited.push((last.0 - 1, last.1)),
        }
    }

    pub fn houses_visited(&self) -> usize {
        self.visited
            .clone()
            .into_iter()
            .collect::<BTreeSet<(i64, i64)>>()
            .len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traverse() {
        let mut santa = Santa::default();
        santa.traverse(Direction::North);
        assert_eq!(santa.visited, vec![(0, 0), (0, 1)]);

        let mut santa = Santa::default();
        santa.traverse(Direction::South);
        assert_eq!(santa.visited, vec![(0, 0), (0, -1)]);

        let mut santa = Santa::default();
        santa.traverse(Direction::East);
        assert_eq!(santa.visited, vec![(0, 0), (1, 0)]);

        let mut santa = Santa::default();
        santa.traverse(Direction::West);
        assert_eq!(santa.visited, vec![(0, 0), (-1, 0)]);
    }

    #[test]
    fn test_houses_visited() {
        let mut santa = Santa::default();
        santa.traverse(Direction::East);
        assert_eq!(santa.houses_visited(), 2);

        let mut santa = Santa::default();
        santa.traverse(Direction::North);
        santa.traverse(Direction::East);
        santa.traverse(Direction::South);
        santa.traverse(Direction::West);
        assert_eq!(santa.houses_visited(), 4);

        let mut santa = Santa::default();
        santa.traverse(Direction::North);
        santa.traverse(Direction::South);
        santa.traverse(Direction::North);
        santa.traverse(Direction::South);
        santa.traverse(Direction::North);
        santa.traverse(Direction::South);
        santa.traverse(Direction::North);
        santa.traverse(Direction::South);
        santa.traverse(Direction::North);
        santa.traverse(Direction::South);
        assert_eq!(santa.houses_visited(), 2);
    }
}
