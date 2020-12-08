use crate::Tile::{Open, Tree};
use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc03.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let map = Map::from_str(&s);
    let traversed_tiles = map.traverse(3);

    let trees = traversed_tiles.iter().filter(|v| **v == Tile::Tree).count();
    eprintln!("Trees: {}", trees);
    assert_eq!(159, trees)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Open,
    Tree,
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn from_str(s: &str) -> Self {
        let width = s.find('\n').unwrap();
        let height = s.len() / (width + 1);
        assert_eq!((width + 1) * height, s.len());

        let mut tiles = Vec::new();
        for row in s.lines() {
            for col in row.chars() {
                match col {
                    '.' => tiles.push(Open),
                    '#' => tiles.push(Tree),
                    _ => unreachable!(),
                }
            }
        }
        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Tile> {
        assert!(y < self.height);
        self.tiles.get((x % self.width) + y * self.width).copied()
    }

    pub fn traverse(&self, dx: usize) -> Vec<Tile> {
        let mut encounters = Vec::new();
        let mut x = 0;
        for y in 0usize..self.height {
            let tile = self.get(x, y).unwrap();

            encounters.push(tile);
            x += dx;
        }
        encounters
    }
}

#[cfg(test)]
mod tests {
    use crate::{Map, Tile};
    const EXAMPLE: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn it_works() {
        let map = Map::from_str(EXAMPLE);

        let trees = map.traverse(3).iter().filter(|v| **v == Tile::Tree).count();
        assert_eq!(7, trees);
    }
}
