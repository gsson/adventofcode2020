#![feature(test)]

use std::io::Read;
use std::fmt::{Debug, Formatter, Write};

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc11.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();
    let mut grid = parse(&input);
    loop {
        let next_grid = grid.simulate();
        if next_grid == grid {
            break;
        }
        grid = next_grid;
    }
    eprintln!("Occupied seats: {}", grid.occupied_seats());
    assert_eq!(2059, grid.occupied_seats());
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Clone, Eq, PartialEq)]
struct Grid {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

fn parse(input: &[u8]) -> Grid {
    let mut rows = input.split(|c| *c == b'\n');
    let first_row = rows.next().unwrap();

    let width = (first_row.len() + 2) as isize;
    let height = (input.len() / (first_row.len() + 1) + 2) as isize;
    let mut tiles = Vec::with_capacity((width * height) as usize);

    tiles.extend(std::iter::repeat(Tile::Floor).take(width as usize));
    tiles.push(Tile::Floor);
    for t in first_row {
        match t {
            b'.' => tiles.push(Tile::Floor),
            b'L' => tiles.push(Tile::EmptySeat),
            b'#' => tiles.push(Tile::OccupiedSeat),
            _ => unreachable!()
        }
    }
    tiles.push(Tile::Floor);

    for row in rows {
        tiles.push(Tile::Floor);
        for t in row {
            match t {
                b'.' => tiles.push(Tile::Floor),
                b'L' => tiles.push(Tile::EmptySeat),
                b'#' => tiles.push(Tile::OccupiedSeat),
                _ => unreachable!()
            }
        }
        tiles.push(Tile::Floor);
    }
    tiles.extend(std::iter::repeat(Tile::Floor).take(width as usize));


    Grid {
        width,
        height,
        tiles,
    }
}

impl Grid {
    fn is_occupied(&self, mut x: isize, mut y: isize, dx: isize, dy: isize) -> usize {
        x += dx;
        y += dy;
        while (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            let i = y * self.width + x;
            let tile = self.tiles[i as usize];
            if tile == Tile::OccupiedSeat {
                return 1;
            } else if tile == Tile::EmptySeat {
                return 0;
            }
            x += dx;
            y += dy;
        }
        0
    }
    fn occupied_neighbors(&self, x: isize, y: isize) -> usize {
        self.is_occupied(x, y, -1, -1)
            + self.is_occupied(x, y, 0, -1)
            + self.is_occupied(x, y, 1, -1)
            + self.is_occupied(x, y, -1, 0)
            + self.is_occupied(x, y, 1, 0)
            + self.is_occupied(x, y, -1, 1)
            + self.is_occupied(x, y, 0, 1)
            + self.is_occupied(x, y, 1, 1)
    }

    fn occupied_seats(&self) -> usize {
        self.tiles.iter()
            .filter(|tile| **tile == Tile::OccupiedSeat)
            .count()
    }

    fn simulate(&self) -> Grid {
        let tiles = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .map(|(x, y)| if self.tiles[(y * self.width + x) as usize] == Tile::Floor {
                Tile::Floor
            } else {
                match self.occupied_neighbors(x, y) {
                    0 => Tile::OccupiedSeat,
                    5..=8 => Tile::EmptySeat,
                    _ => self.tiles[(y * self.width + x) as usize]
                }
            })
            .collect();
        Grid {
            width: self.width,
            height: self.height,
            tiles,
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
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

    const OUTPUTS: [&str; 6] = ["#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
    "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"];

    #[test]
    fn test_simulation() {
        let mut grid = parse(INPUT1.as_bytes());
        for output in &OUTPUTS {
            eprintln!("{:?}", grid);
            grid = grid.simulate();
            let o = parse(output.as_bytes());
            assert_eq!(o, grid);
        }
    }


    #[test]
    fn find_steady_state() {
        let mut grid = parse(INPUT1.as_bytes());
        loop {
            let next_grid = grid.simulate();
            if next_grid == grid {
                break;
            }
            grid = next_grid;
        }
        assert_eq!(26, grid.occupied_seats());
    }

    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc11.txt").unwrap();
        let mut input = Vec::new();
        f.read_to_end(&mut input).unwrap();
        bencher.iter(move || {
            let mut grid = parse(&input);

            loop {
                let next_grid = grid.simulate();
                if next_grid == grid {
                    break;
                }
                grid = next_grid;
            };
            grid.occupied_seats()
        });
    }



    #[test]
    fn test_occupied_neighbors_1() {
        let grid = parse(".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....".as_bytes());
        assert_eq!(8, grid.occupied_neighbors(4, 5));
    }

    #[test]
    fn test_occupied_neighbors_2() {
        let grid = parse(".............
.L.L.#.#.#.#.
.............".as_bytes());
        assert_eq!(0, grid.occupied_neighbors(2, 2));
    }

    #[test]
    fn test_occupied_neighbors_3() {
        let grid = parse(".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.".as_bytes());
        assert_eq!(Tile::EmptySeat, grid.tiles[(4 * grid.width + 4) as usize]);
        assert_eq!(0, grid.occupied_neighbors(4, 4));
    }

}
