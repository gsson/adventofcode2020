#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc06.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let result = solve(&s);
    eprintln!("{}", result);
    assert_eq!(3288, result);
}

fn solve(s: &str) -> u32 {
    s.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|s| {
                    s.chars()
                        .fold(u128::min_value(), |a, c| a | 1 << (c as i32))
                })
                .fold(u128::max_value(), |a, b| a & b)
                .count_ones()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::solve;

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b
";

    #[test]
    fn test_seat() {
        assert_eq!(6, solve(EXAMPLE));
    }
}
