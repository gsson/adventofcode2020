#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;
use std::collections::HashMap;
use std::fmt::{Formatter, Debug};

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc14.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let mut mask = Mask::default();
    let mut mem: HashMap<usize, Value> = HashMap::new();
    input.lines()
        .map(Instruction::new)
        .for_each(|i| i.apply(&mut mask, &mut mem));
    let out = mem.values().map(|v| v.0)
        .sum::<usize>();

    eprintln!("{}", out);
    assert_eq!(14722016054794, out);
}

const MAX: usize = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111usize;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct Value(usize);

impl Value {
    fn new(s: &str) -> Self {
        Self(usize::from_str_radix(s, 10).unwrap() & MAX)
    }
    fn new_from_binary(s: &str) -> Self {
        Self(usize::from_str_radix(s, 2).unwrap() & MAX)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:036b} ({})", self.0, self.0))
    }
}



#[derive(Copy, Clone, Default, Debug)]
struct Mask(usize, usize);

impl Mask {
    fn new(s: &str) -> Self {
        let mut and_mask = 0;
        let mut or_mask = 0;
        for (i, c) in s.chars().enumerate() {
            let bit = 1usize << (s.len() - i - 1);

            match c {
                '1' => { or_mask |= bit; }
                '0' => {}
                'X' => { and_mask |= bit; }
                _ => unreachable!()
            }
        }
        Self(and_mask & MAX, or_mask & MAX)
    }

    fn apply(&self, v: Value) -> Value {
        Value(v.0 & self.0 | self.1)
    }
}

enum Instruction {
    Mask(Mask),
    Write(usize, Value),
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" = ").unwrap();
        if lhs == "mask" {
            Self::Mask(Mask::new(rhs))
        } else {
            let pos = lhs[4..lhs.len() - 1].parse::<usize>().unwrap();
            Self::Write(pos, Value::new(rhs))
        }
    }
    fn apply<'a>(&self, mask: &'a mut Mask, mem: &'a mut HashMap<usize, Value>) {
        match self {
            Instruction::Mask(m) => *mask = *m,
            Instruction::Write(address, value) => {
                mem.insert(*address, mask.apply(*value));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;
    use std::hash::Hash;

    const EXAMPLE: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_program() {
        let mut mask = Mask::default();
        let mut mem: HashMap<usize, Value> = HashMap::new();
        EXAMPLE.lines()
            .map(Instruction::new)
            .for_each(|i| i.apply(&mut mask, &mut mem));
        let out = mem.values().map(|v| v.0)
            .sum::<usize>();

        eprintln!("{}", out);
        assert_eq!(165, out);
    }

    const MASK_EXAMPLES: [(&str, &str, &str); 3] = [
        ("000000000000000000000000000000001011",
         "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
         "000000000000000000000000000001001001"),
        ("000000000000000000000000000001100101",
         "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
         "000000000000000000000000000001100101"),
        ("000000000000000000000000000000000000",
         "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
         "000000000000000000000000000001000000"),
    ];

    #[test]
    fn test_mask() {
        for (value, mask, expected) in &MASK_EXAMPLES {
            assert_eq!(Value::new_from_binary(*expected),
                       Mask::new(*mask)
                           .apply(Value::new_from_binary(*value)));

        }
    }

}
