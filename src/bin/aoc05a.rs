#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc05.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let max = s.lines().map(|s| Seat::from_str(s)).max().unwrap();

    eprintln!("Seat id: {}", max.id());
    assert_eq!(832, max.id())
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Seat(u8, u8);

impl Seat {
    fn id(&self) -> usize {
        self.row() << 3 | self.col()
    }

    fn row(&self) -> usize {
        self.0 as usize
    }

    fn col(&self) -> usize {
        self.1 as usize
    }

    fn from_str(s: &str) -> Self {
        fn apply(s: Seat, c: char) -> Seat {
            match c {
                'F' => Seat(s.0 << 1, s.1),
                'B' => Seat(s.0 << 1 | 1, s.1),
                'L' => Seat(s.0, s.1 << 1),
                'R' => Seat(s.0, s.1 << 1 | 1),
                _ => unreachable!(),
            }
        }

        s.chars().fold(Seat(0, 0), apply)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::Seat;

    const EXAMPLES: [(&str, usize); 4] = [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];

    #[test]
    fn test_seat() {
        for (s, id) in &EXAMPLES {
            assert_eq!(*id, Seat::from_str(s).id());
        }
    }
}
