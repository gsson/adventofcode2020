#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;
use crate::State::{Inactive, Active};
use std::fmt::{Debug, Formatter, Write};
use std::convert::TryInto;
use std::ops::{RangeInclusive, RangeBounds, Bound};

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc17.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();
    let a = PocketDimension::new(&input, 6);
    let b = a.clone();
    let o = (0..6)
        .fold((a, b), |(a, mut b), _| { a.simulate_into(&mut b); (b, a) } );

    eprintln!("{}", o.0.active_cubes());
    assert_eq!(2332, o.0.active_cubes());
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Inactive,
    Active,
}

#[derive(Copy, Clone, Debug)]
struct Extent(usize, usize);
impl Extent {
    fn new<T: RangeBounds<usize>>(bounds: T) -> Self {
        let start = match bounds.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => n + 1,
            Bound::Unbounded => unreachable!()
        };
        let end = match bounds.end_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => n - 1,
            Bound::Unbounded => unreachable!()
        };
        Self(start, end)
    }
    fn grow(self) -> Self {
        Extent(self.0 - 1, self.1 + 1)
    }
}
impl IntoIterator for Extent {
    type Item = usize;
    type IntoIter = RangeInclusive<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0 ..= self.1
    }
}

#[derive(Clone)]
struct PocketDimension {
    x_range: Extent,
    y_range: Extent,
    z_range: Extent,
    w_range: Extent,

    height_stride: usize,
    depth_stride: usize,
    w_stride: usize,
    neighbours: [usize; 80],
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
        let w = (1 + 2) + max_growth;
        let height_stride = width;
        let depth_stride = height_stride * height;
        let w_stride = depth_stride * depth;
        let neighbours = (-1..=1)
            .flat_map(|z| (-1..=1).map(move |y| (y, z)))
            .flat_map(|(y, z)| (-1..=1).map(move |x| (x, y, z)))
            .flat_map(|(x, y, z)| (-1..=1).map(move |w| (x, y, z, w)))
            .filter(|(x, y, z, w)| *x != 0 || *y != 0 || *z != 0 || *w != 0)
            .map(|(x, y, z, w)| x + (y * height_stride as isize) + (z * depth_stride as isize) + (w * w_stride as isize))
            .map(|o| o as usize)
            .collect::<Vec<_>>();
        let neighbours: [usize; 80] = neighbours.try_into().unwrap();

        let mut cubes = vec![Inactive; w_stride * w];
        let x0 = (width - input_width) / 2;
        let y0 = (height - input_height) / 2;
        let z0 = depth / 2;
        let w0 = w / 2;

        for v in 0..input_height {
            for u in 0..input_width {
                let state = match s[v * (input_width + 1) + u] {
                    b'.' => Inactive,
                    b'#' => Active,
                    _ => unreachable!()
                };
                cubes[(x0 + u) + (y0 + v) * height_stride + (z0 * depth_stride) + (w0 * w_stride)] = state;
            }
        }

        let x_range = Extent::new(x0 ..= x0 + input_width - 1);
        let y_range = Extent::new(y0 ..= y0 + input_height - 1);
        let z_range = Extent::new(z0 ..= z0);
        let w_range = Extent::new(w0 ..= w0);

        Self {
            x_range,
            y_range,
            z_range,
            w_range,
            depth_stride,
            height_stride,
            w_stride,
            neighbours,
            cubes,
        }
    }

    fn active_neighbors(&self, o: usize) -> usize {
        self.neighbours.iter()
            .filter(|n| self.cubes[o.wrapping_add(**n)] == Active)
            .count()
    }

    fn simulate(&self) -> Self {
        let mut cubes = vec![Inactive; self.cubes.len()];

        let x_range = self.x_range.grow();
        let y_range = self.y_range.grow();
        let z_range = self.z_range.grow();
        let w_range = self.w_range.grow();

        for w in w_range {
            let wo = w * self.w_stride;
            for z in z_range {
                let zo = z * self.depth_stride;
                for y in y_range {
                    let yo = y * self.height_stride;
                    for x in x_range {
                        let o = x + yo + zo + wo;
                        let next_state = match (self.cubes[o], self.active_neighbors(o)) {
                            (Active, 2) | (Active, 3) => Active,
                            (Inactive, 3) => Active,
                            _ => Inactive,
                        };
                        cubes[o] = next_state
                    }
                }
            }
        }

        Self {
            cubes,
            x_range,
            y_range,
            z_range,
            w_range,
            ..*self
        }
    }

    fn simulate_into(&self, dst: &mut Self) {
        let x_range = self.x_range.grow();
        let y_range = self.y_range.grow();
        let z_range = self.z_range.grow();
        let w_range = self.w_range.grow();
        for w in w_range.clone() {
            let wo = w * self.w_stride;
            for z in z_range.clone() {
                let zo = z * self.depth_stride;
                for y in y_range.clone() {
                    let yo = y * self.height_stride;
                    for x in x_range.clone() {
                        let o = x + yo + zo + wo;
                        let next_state = match (self.cubes[o], self.active_neighbors(o)) {
                            (Active, 2) | (Active, 3) => Active,
                            (Inactive, 3) => Active,
                            _ => Inactive,
                        };
                        dst.cubes[o] = next_state
                    }
                }
            }
        }
        dst.x_range = x_range;
        dst.y_range = y_range;
        dst.z_range = z_range;
        dst.w_range = w_range;
    }

    fn active_cubes(&self) -> usize {
        self.cubes.iter().filter(|s| **s == Active).count()
    }
}

impl Debug for PocketDimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for w in self.w_range {
            let wo = w * self.w_stride;
            for z in self.z_range {
                let zo = z * self.depth_stride;

                write!(f, "\nz = {}, w = {}", z, w)?;
                for y in self.y_range {
                    f.write_char('\n')?;
                    let yo = y * self.height_stride;
                    for x in self.x_range {
                        let o = x + yo + zo + wo;
                        match self.cubes[o] {
                            Inactive => f.write_char('.')?,
                            Active => f.write_char('#')?
                        }
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
    }

    #[test]
    fn test_6() {
        let init = PocketDimension::new(EXAMPLE, 6);
        let o = (0..6).fold(init, |pd, _| pd.simulate() );
        eprintln!("{}", o.active_cubes());
        assert_eq!(848, o.active_cubes());
    }

    #[test]
    fn test_6_double_buffer() {
        let a = PocketDimension::new(EXAMPLE, 6);
        let b = a.clone();
        let o = (0..6).fold((a, b), |(a, mut b), _| { a.simulate_into(&mut b); (b, a) } );
        eprintln!("{}", o.0.active_cubes());
        assert_eq!(848, o.0.active_cubes());
    }

    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc17.txt").unwrap();
        let mut input = Vec::new();
        f.read_to_end(&mut input).unwrap();

        bencher.iter(move || (0..6)
            .fold(PocketDimension::new(&input, 6), |a, _| a.simulate()));
    }


    #[bench]
    fn benchmark_double_buffer(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc17.txt").unwrap();
        let mut input = Vec::new();
        f.read_to_end(&mut input).unwrap();

        bencher.iter(move || {
            let a = PocketDimension::new(EXAMPLE, 6);
            let b = a.clone();
            (0..6)
                .fold((a, b), |(a, mut b), _| { a.simulate_into(&mut b); (b, a) } )
        });
    }
}
