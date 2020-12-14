#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc12.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let distance = parse(&input)
        .fold(Ship::new(), Ship::apply)
        .distance();

    eprintln!("Distance moved: {}", distance);
    assert_eq!(1645, distance);
}

enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    TurnLeft(isize),
    TurnRight(isize),
    Forward(isize),
}

#[derive(Copy, Clone)]
struct Ship {
    heading: isize,
    x: isize,
    y: isize,
}

fn parse(input: &str) -> impl Iterator<Item=Instruction> + '_ {
    fn parse_instruction(i: &str) -> Instruction {
        match i.split_at(1) {
            ("N", n) => Instruction::North(n.parse().unwrap()),
            ("S", n) => Instruction::South(n.parse().unwrap()),
            ("E", n) => Instruction::East(n.parse().unwrap()),
            ("W", n) => Instruction::West(n.parse().unwrap()),
            ("L", n) => Instruction::TurnLeft(n.parse().unwrap()),
            ("R", n) => Instruction::TurnRight(n.parse().unwrap()),
            ("F", n) => Instruction::Forward(n.parse().unwrap()),
            _ => unreachable!()
        }
    }
    input.lines()
        .map(parse_instruction)
}

impl Ship {
    fn new() -> Self {
        Self {
            heading: 90,
            x: 0,
            y: 0
        }
    }

    fn apply(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::North(distance) => Self { y: self.y + distance, .. self },
            Instruction::South(distance) => Self { y: self.y - distance, .. self },
            Instruction::East(distance) => Self { x: self.x + distance, .. self },
            Instruction::West(distance) => Self { x: self.x - distance, .. self },
            Instruction::TurnLeft(turn_by) => Self { heading: (self.heading - turn_by).rem_euclid(360), .. self },
            Instruction::TurnRight(turn_by) => Self { heading: (self.heading + turn_by).rem_euclid(360), .. self },
            Instruction::Forward(distance) => {
                match self.heading {
                    0 => Self { y: self.y + distance, .. self },
                    90 => Self { x: self.x + distance, .. self },
                    180 => Self { y: self.y - distance, .. self },
                    270 => Self { x: self.x - distance, .. self },
                    _ => unreachable!()
                }
            },
        }
    }

    fn distance(self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE: &str = "F10
N3
F7
R90
F11
";

    #[test]
    fn test_simulation() {
        let distance = parse(EXAMPLE)
            .fold(Ship::new(), Ship::apply)
            .distance();
        assert_eq!(25, distance);
    }

}
