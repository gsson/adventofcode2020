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
    assert_eq!(35292, distance);
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
    x: isize,
    y: isize,
    wx: isize,
    wy: isize
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
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    fn turn(self, turn_by: isize) -> Self {
        match turn_by.rem_euclid(360) {
            0 => self,
            90 => Self { wx: self.wy, wy: -self.wx, .. self },
            180 => Self { wx: -self.wx, wy: -self.wy, .. self },
            270 => Self { wx: -self.wy, wy: self.wx, .. self },
            _ => unreachable!()
        }
    }

    fn apply(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::North(distance) => Self { wy: self.wy + distance, .. self },
            Instruction::South(distance) => Self { wy: self.wy - distance, .. self },
            Instruction::East(distance) => Self { wx: self.wx + distance, .. self },
            Instruction::West(distance) => Self { wx: self.wx - distance, .. self },
            Instruction::TurnLeft(turn_by) => self.turn(-turn_by),
            Instruction::TurnRight(turn_by) => self.turn(turn_by),
            Instruction::Forward(distance) => Self { x: self.x + self.wx * distance, y: self.y + self.wy * distance, .. self },
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
        assert_eq!(286, distance);
    }
}
