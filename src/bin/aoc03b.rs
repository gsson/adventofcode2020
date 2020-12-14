#![feature(test)]

use crate::Tile::{Open, Tree};
use std::io::Read;

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc03.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let map = Map::from_str(&s);
    let trees = find_trees(&map);
    eprintln!("Trees: {}", trees);
    assert_eq!(6419669520, trees);
}

fn find_trees(map: &Map) -> usize {
    SLOPES
        .iter()
        .map(|(dx, dy)| map.traverse(*dx, *dy).filter(|v| *v == Tile::Tree).count())
        .product::<usize>()
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

    pub fn gen(height: usize, dx: usize, dy: usize) -> impl Iterator<Item = (usize, usize)> {
        let mut x = 0;
        let mut y = 0;
        std::iter::from_fn(move || {
            if y < height {
                let r = Some((x, y));

                x += dx;
                y += dy;
                r
            } else {
                None
            }
        })
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        assert!(y < self.height);
        self.tiles.get((x % self.width) + y * self.width).copied().unwrap()
    }


    pub fn traverse(&self, dx: usize, dy: usize) -> impl Iterator<Item = Tile> + '_ {
        Self::gen(self.height, dx, dy)
            .map(move |(x, y)| self.get(x, y))
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use crate::{find_trees, Map};
    use std::io::Read;
    use test::bench::Bencher;

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

        let trees = find_trees(&map);

        assert_eq!(336, trees);
    }

    #[bench]
    fn bench_traverse(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc03.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        let map = Map::from_str(&s);

        bencher.iter(move || find_trees(&map))
    }
}
