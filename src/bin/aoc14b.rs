#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

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
    assert_eq!(3618217244644, out);
}

const MAX: usize = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111usize;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
struct Value(usize);

impl Value {
    fn new(s: &str) -> Self {
        Self(s.parse::<usize>().unwrap() & MAX)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:036b} ({})", self.0, self.0))
    }
}

#[derive(Clone, Default)]
struct Mask(usize, usize, Vec<usize>);

impl Debug for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("or: {:36b}, and: {:36b} floating: {:?}", self.0, self.1, self.2))
    }
}

impl Mask {
    fn new(s: &str) -> Self {
        let mut or_mask = 0;
        let mut and_mask = 0;
        let mut floating = Vec::new();
        for (i, c) in s.chars().enumerate() {
            let bit = 1usize << (s.len() - i - 1);
            match c {
                '1' => {
                    or_mask |= bit;
                    and_mask |= bit;
                },
                '0' => { and_mask |= bit; },
                'X' => { floating.push(bit); }
                _ => unreachable!()
            }
        }
        floating.sort_unstable();
        Self(or_mask & MAX, and_mask & MAX, floating)
    }

    fn bits(mut n: usize) -> impl Iterator<Item=usize> {
        std::iter::from_fn(move || {
            if n != 0 {
                let b = n.trailing_zeros();
                n ^= 1 << b;
                Some(b as usize)
            } else {
                None
            }
        })
    }

    fn floating(&self) -> impl Iterator<Item=usize> + '_ {
        (0..1 << self.2.len())
            .map(move |i| Self::bits(i)
                .fold(0usize, |a, n| a | self.2[n]))
    }
    fn apply(&self, address: usize) -> impl Iterator<Item=usize> + '_ {
        let address = address & self.1 | self.0;
        self.floating()
            .map(move |f| address | f)
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
            Instruction::Mask(m) => *mask = m.clone(),
            Instruction::Write(address, value) => {
                mask.apply(*address)
                    .for_each(|a| {
                        mem.insert(a, *value);
                    });
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

    const EXAMPLE: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

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
        assert_eq!(208, out);
    }

    #[test]
    fn test_mask() {
        let mask_examples: [(&str, usize, Vec<usize>); 2] = [
            ("000000000000000000000000000000X1001X",
             42,
             vec![26, 27, 58, 59]),
            ("00000000000000000000000000000000X0XX",
             26,
             vec![16, 17, 18, 19, 24, 25, 26, 27]),
        ];
        for (mask, address, expected) in &mask_examples {
            let mask = Mask::new(*mask);
            eprintln!("{:?}", mask);
            let v = mask
                .apply(*address)
                .collect::<Vec<_>>();
            assert_eq!(*expected, v);
        }
    }
}
