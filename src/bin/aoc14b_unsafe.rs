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
    let mut mem: HashMap<Address, Value> = HashMap::new();
    input.lines()
        .map(Instruction::new)
        .for_each(|i| i.apply(&mut mask, &mut mem));
    let out = mem.values().map(|v| v.0)
        .sum::<u64>();

    eprintln!("{}", out);
    assert_eq!(3618217244644, out);
}

const MAX: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111u64;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Value(u64);

impl Value {
    fn new(s: &str) -> Self {
        Self(s.parse::<u64>().unwrap() & MAX)
    }
}


impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:036b} ({})", self.0, self.0))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Address(u64);

impl Address {
    fn new(s: &str) -> Self {
        Self(s.parse::<u64>().unwrap() & MAX)
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:x}", self.0))
    }
}


#[derive(Clone, Copy, Default)]
struct Mask(u64, u64);

impl Debug for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("or: {:36b}, floating: {:?}", self.0, self.1))
    }
}

impl Mask {
    fn new(s: &str) -> Self {
        let mut or_mask = 0;
        let mut floating_mask = 0;
        for (i, c) in s.chars().enumerate() {
            let bit = 1u64 << (s.len() - i - 1);
            match c {
                '1' => { or_mask |= bit; },
                '0' => { },
                'X' => { floating_mask |= bit; }
                _ => unreachable!()
            }
        }
        Self(or_mask & MAX, floating_mask & MAX)
    }

    fn floating_bits_permutation(&self, n: u64) -> u64 {
        unsafe {
            core::arch::x86_64::_pdep_u64(n, self.1 as u64)
        }
    }

    fn floating_bits(&self) -> impl Iterator<Item=u64> + '_ {
        (0..1 << self.1.count_ones())
            .map(move |i| self.floating_bits_permutation(i))
    }

    fn apply(&self, address: Address) -> impl Iterator<Item=Address> + '_ {
        let address = address.0 | self.0;
        self.floating_bits()
            .map(move |f| Address(address ^ f))
    }
}

enum Instruction {
    Mask(Mask),
    Write(Address, Value),
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" = ").unwrap();
        if lhs == "mask" {
            Self::Mask(Mask::new(rhs))
        } else {
            Self::Write(Address::new(&lhs[4..lhs.len() - 1]), Value::new(rhs))
        }
    }

    fn apply<'a>(&self, mask: &'a mut Mask, mem: &'a mut HashMap<Address, Value>) {
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

    const EXAMPLE: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_program() {
        let mut mask = Mask::default();
        let mut mem: HashMap<Address, Value> = HashMap::new();
        EXAMPLE.lines()
            .map(Instruction::new)
            .for_each(|i| i.apply(&mut mask, &mut mem));
        let out = mem.values().map(|v| v.0)
            .sum::<u64>();

        eprintln!("{}", out);
        assert_eq!(208, out);
    }

    #[test]
    fn test_mask() {
        let mask_examples: [(&str, u64, Vec<u64>); 2] = [
            ("000000000000000000000000000000X1001X",
             42,
             vec![26, 27, 58, 59]),
            ("00000000000000000000000000000000X0XX",
             26,
             vec![16, 17, 18, 19, 24, 25, 26, 27]),
        ];
        for (mask, address, expected) in &mask_examples {
            let mask = Mask::new(*mask);

            assert!(mask
                .apply(Address(*address))
                .all(|n| expected.contains(&n.0)));
        }
    }
}
