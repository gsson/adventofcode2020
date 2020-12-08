#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc05.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut seats = [0u8; 128];

    s.lines()
        .map(|s| Seat::from_str(s))
        .for_each(|seat| seats[seat.row] |= 1u8 << seat.col);

    let seat = seats
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, _)| *i > 0 && *i < 127)
        .filter(|(_, row)| row.count_zeros() == 1)
        .filter(|(i, row)| seats[i - 1] & !*row == !*row && seats[i + 1] & !*row == !*row)
        .find_map(|(i, row)| Some((i, (row.trailing_ones()) as usize)))
        .map(|(row, col)| Seat { row, col })
        .unwrap();

    eprintln!("{}", seat.id());
    assert_eq!(517, seat.id());
}

#[derive(Default, Ord, PartialOrd, Eq, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row << 3 | self.col
    }

    fn from_str(s: &str) -> Self {
        fn apply(s: Seat, c: char) -> Seat {
            match c {
                'F' => Seat {
                    row: s.row << 1,
                    ..s
                },
                'B' => Seat {
                    row: s.row << 1 | 1,
                    ..s
                },
                'L' => Seat {
                    col: s.col << 1,
                    ..s
                },
                'R' => Seat {
                    col: s.col << 1 | 1,
                    ..s
                },
                _ => unreachable!(),
            }
        }

        s.chars().fold(Seat::default(), apply)
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
