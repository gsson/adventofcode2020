#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;
use crate::State::{Inactive, Active};
use std::fmt::{Debug, Formatter, Write};
use std::convert::TryInto;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc17.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();

    let o = (0..6).fold(PocketDimension::new(&input, 6), |a, _| a.simulate());
    eprintln!("{}", o.active_cubes());
    assert_eq!(380, o.active_cubes());
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Inactive,
    Active,
}

struct PocketDimension {
    width: usize,
    height: usize,
    depth: usize,
    height_stride: usize,
    depth_stride: usize,
    neighbours: [usize; 26],
    cubes: Vec<State>,
}

impl PocketDimension {
    fn new(s: &[u8], max_cycles: usize) -> Self {
        let input_width = s.iter().position(|b| *b == b'\n').unwrap();
        let input_height = s.len() / (input_width + 1);

        let max_growth = 2 * max_cycles;

        let width = (input_width + 2) + max_growth;
        let height = (input_height + 2) + max_growth;
        let depth = (1 + 2) + max_growth;
        let height_stride = width;
        let depth_stride = height_stride * height;
        let neighbours = (-1..=1)
            .flat_map(|z| (-1..=1).map(move |y| (y, z)))
            .flat_map(|(y, z)| (-1..=1).map(move |x| (x, y, z)))
            .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
            .map(|(x, y, z)| x + (y * height_stride as isize) + (z * depth_stride as isize))
            .map(|o| o as usize)
            .collect::<Vec<_>>();
        let neighbours: [usize; 26] = neighbours.try_into().unwrap();

        let mut cubes = vec![Inactive; depth_stride * depth];
        let x0 = (width - input_width) / 2;
        let y0 = (height - input_height) / 2;
        let z0 = depth / 2;


        for v in 0..input_height {
            for u in 0..input_width {
                let state = match s[v * (input_width + 1) + u] {
                    b'.' => Inactive,
                    b'#' => Active,
                    _ => unreachable!()
                };
                cubes[(x0 + u) + (y0 + v) * height_stride + (z0 * depth_stride)] = state;
            }
        }

        Self {
            width,
            height,
            depth,
            depth_stride,
            height_stride,
            neighbours,
            cubes
        }
    }

    fn active_neighbors(&self, o: usize) -> usize {
        self.neighbours.iter()
            .filter(|n| self.cubes[o.wrapping_add(**n)] == Active)
            .count()
    }

    fn simulate(&self) -> Self {
        let mut cubes = vec![Inactive; self.depth_stride * self.depth];

        for z in 1..self.depth - 1 {
            let zo = z * self.depth_stride;
            for y in 1..self.height - 1 {
                let yo = y * self.height_stride;
                for x in 1..self.width - 1 {
                    let o = x + yo + zo;
                    let next_state = match (self.cubes[o], self.active_neighbors(o)) {
                        (Active, 2) | (Active, 3) => Active,
                        (Inactive, 3) => Active,
                        _ => Inactive,
                    };
                    cubes[o] = next_state
                }
            }
        }

        Self {
            cubes,
            .. *self
        }

    }

    fn active_cubes(&self) -> usize {
        self.cubes.iter().filter(|s| **s == Active).count()
    }
}

impl Debug for PocketDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for z in 0..self.depth {
            write!(f, "\nz = {}", z)?;
            let zo = z * self.depth_stride;
            for y in 0..self.height {
                f.write_char('\n')?;
                let yo = y * self.height_stride;
                for x in 0..self.width {
                    let o = x + yo + zo;
                    match self.cubes[o] {
                        Inactive => f.write_char('.')?,
                        Active => f.write_char('#')?
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE: &[u8] = b".#.
..#
###
";

    #[test]
    fn test_steps() {
        let example = PocketDimension::new(EXAMPLE, 6);
        eprintln!("===\n0\n{:?}", example);
        let example = example.simulate();
        eprintln!("===\n1\n{:?}", example);
        let example = example.simulate();
        eprintln!("===\n2\n{:?}", example);
        let example = example.simulate();
        eprintln!("===\n3\n{:?}", example);
        let example = example.simulate();
        let example = example.simulate();
        let example = example.simulate();
        eprintln!("===\n{}", example.active_cubes());

    }

    #[test]
    fn test_6() {
        let o = (0..6).fold(PocketDimension::new(EXAMPLE, 6), |a, _| a.simulate());
        eprintln!("{}", o.active_cubes());
        assert_eq!(112, o.active_cubes());
    }

    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc17.txt").unwrap();
        let mut input = Vec::new();
        f.read_to_end(&mut input).unwrap();

        bencher.iter(move || (0..6)
            .fold(PocketDimension::new(&input, 6), |a, _| a.simulate()));
    }
}
