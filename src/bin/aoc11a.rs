#![feature(test)]

use std::io::Read;
use std::fmt::{Debug, Formatter, Write};

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc11.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let mut grid = parse(&input);
    loop {
        let next_grid = grid.simulate();
        if next_grid == grid {
            break;
        }
        grid = next_grid;
    }
    eprintln!("Occupied seats: {}", grid.occupied_seats());
    assert_eq!(2289, grid.occupied_seats());

}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat
}

#[derive(Clone, Eq, PartialEq)]
struct Grid {
    width: isize,
    height: isize,
    neighbor_offsets: [isize; 8],
    tiles: Vec<Tile>
}

fn parse(input: &str) -> Grid {
    let rows = input.lines()
        .collect::<Vec<_>>();

    let width = (rows[0].len() + 2) as isize;
    let height = (rows.len() + 2) as isize;
    let mut tiles = Vec::with_capacity((width * height) as usize);
    tiles.extend(std::iter::repeat(Tile::Floor).take(width as usize));
    for row in rows {
        tiles.push(Tile::Floor);
        for t in row.chars() {
            match t {
                '.' => tiles.push(Tile::Floor),
                'L' => tiles.push(Tile::EmptySeat),
                '#' => tiles.push(Tile::OccupiedSeat),
                _ => unreachable!()
            }
        }
        tiles.push(Tile::Floor);
    }
    tiles.extend(std::iter::repeat(Tile::Floor).take(width as usize));


    Grid {
        width,
        height,
        neighbor_offsets: [
            -width - 1, -width, -width + 1,
            -1, 1,
            width - 1, width, width + 1
        ],
        tiles
    }
}

impl Grid {
    fn occupied_neighbors(&self, c: isize) -> usize {
        self.neighbor_offsets.iter()
            .map(|o| c + o)
            .filter(|i| self.tiles[*i as usize] == Tile::OccupiedSeat)
            .count()
    }

    fn occupied_seats(&self) -> usize {
        self.tiles.iter()
            .filter(|tile| **tile == Tile::OccupiedSeat)
            .count()
    }

    fn simulate(&self) -> Grid {
        let tiles = self.tiles.iter()
            .enumerate()
            .map(|(i, t)| if *t == Tile::Floor {
                Tile::Floor
            } else {
                match self.occupied_neighbors(i as isize) {
                    0 => Tile::OccupiedSeat,
                    4 ..= 8 => Tile::EmptySeat,
                    _ => *t
                }
            })
            .collect();
        Grid {
            width: self.width,
            height: self.height,
            neighbor_offsets: self.neighbor_offsets.clone(),
            tiles,
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 1 .. self.height - 1 {
            for x in 1 .. self.width - 1 {
                match self.tiles[(x + y * self.width) as usize] {
                    Tile::Floor => f.write_char('.')?,
                    Tile::EmptySeat => f.write_char('L')?,
                    Tile::OccupiedSeat => f.write_char('#')?,
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const INPUT1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    const OUTPUTS: [&str; 5] = ["#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
    "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
    "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
    "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
    "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"];

    #[test]
    fn test_simulation() {
        let mut grid = parse(INPUT1);
        for output in &OUTPUTS {
            eprintln!("{:?}", grid);
            grid = grid.simulate();
            let o = parse(*output);
            assert_eq!(o, grid);
        }
    }


    #[test]
    fn find_steady_state() {
        let mut grid = parse(INPUT1);
        loop {
            let next_grid = grid.simulate();
            if next_grid == grid {
                break;
            }
            grid = next_grid;
        }
        assert_eq!(37, grid.occupied_seats());
    }

}
