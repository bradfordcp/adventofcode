#![feature(test)]

use std::fs;

#[derive(Debug, Clone)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => panic!("Invalid character {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn height(self: &Self) -> usize {
        self.tiles.len() / self.width
    }
}

#[derive(Debug, Clone)]
struct Slope {
    right: usize,
    down: usize,
}

#[allow(dead_code)]
fn read_map(path: &str) -> Option<Map> {
    if let Ok(input) = fs::read_to_string(path) {
        let width = input.find("\n")?;
        let tiles: Vec<Tile> = input
            .chars()
            .filter(|c| c != &'\n')
            .map(|c| c.into())
            .collect();

        Some(Map {
            width: width,
            tiles: tiles,
        })
    } else {
        None
    }
}

#[allow(dead_code)]
fn tree_encounters(map: Map, slope: Slope) -> usize {
    (0..(map.height() / slope.down))
        .map(|level| {
            let x = {
                let offset_x = slope.right * level;
                if offset_x >= map.width {
                    offset_x % map.width
                } else {
                    offset_x
                }
            };
            let y = slope.down * level;
            let index = x + (y * map.width);

            println!("l: {}, x: {}, y: {}, i: {}", level, x, y, index);

            map.tiles.get(index)
        })
        .filter(|tile| match tile {
            Some(Tile::Tree) => true,
            Some(Tile::Open) => false,
            None => false,
        })
        .count()
}

#[allow(dead_code)]
fn slope_products(map: Map) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .into_iter()
        .map(|slope| {
            println!("------------------");
            println!("{:#?}", slope);
            println!("Map:\n  height: {}, width: {}", map.height(), map.width);
            tree_encounters(map.clone(), slope)
        })
        .fold(1, |acc, ele| {
            println!("acc: {}, ele: {}", acc, ele);
            acc * ele
        })
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn test_read_sample_map() {
        let map = read_map("test/sample");
        assert!(map.is_some());
        assert_eq!(map.clone().unwrap().tiles.len(), 121);
        assert_eq!(map.clone().unwrap().width, 11);
    }

    #[test]
    fn test_read_input_map() {
        let map = read_map("test/input");
        assert!(map.is_some());
        assert_eq!(map.clone().unwrap().tiles.len(), 10013);
        assert_eq!(map.clone().unwrap().width, 31);
    }

    #[test]
    fn test_sample_tree_encounters() {
        let map = read_map("test/sample").unwrap();
        let slope = Slope { right: 3, down: 1 };
        let encounters = tree_encounters(map, slope);

        assert_eq!(encounters, 7);
    }

    #[test]
    fn test_input_tree_encounters() {
        let map = read_map("test/input").unwrap();
        let slope = Slope { right: 3, down: 1 };
        let encounters = tree_encounters(map, slope);

        assert_eq!(encounters, 162);
    }

    #[test]
    fn test_sample_slope_products() {
        let map = read_map("test/sample").unwrap();
        assert_eq!(slope_products(map), 336);
    }

    #[test]
    fn test_input_slope_products() {
        let map = read_map("test/input").unwrap();
        assert_eq!(slope_products(map), 3064612320);
    }

    #[bench]
    fn bench_sample_read_map(b: &mut Bencher) {
        b.iter(|| {
            read_map("test/sample").unwrap();
        })
    }

    #[bench]
    fn bench_input_read_map(b: &mut Bencher) {
        b.iter(|| {
            read_map("test/input").unwrap();
        })
    }

    #[bench]
    fn bench_sample_tree_encounters(b: &mut Bencher) {
        let map = read_map("test/sample").unwrap();
        let slope = Slope { right: 3, down: 1 };

        b.iter(|| {
            tree_encounters(map.clone(), slope.clone());
        })
    }

    #[bench]
    fn bench_input_tree_encounters(b: &mut Bencher) {
        let map = read_map("test/input").unwrap();
        let slope = Slope { right: 3, down: 1 };

        b.iter(|| {
            tree_encounters(map.clone(), slope.clone());
        })
    }

    #[bench]
    fn bench_sample_slope_products(b: &mut Bencher) {
        let map = read_map("test/sample").unwrap();

        b.iter(|| {
            slope_products(map.clone());
        })
    }

    #[bench]
    fn bench_input_slope_products(b: &mut Bencher) {
        let map = read_map("test/input").unwrap();

        b.iter(|| {
            slope_products(map.clone());
        })
    }
}
