#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc06.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let x = s
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(Questionnaire::from_str)
                .fold(Questionnaire(0), |q1, q2| q1.merge(q2))
                .count()
        })
        .sum::<usize>();
    eprintln!("{}", x);
    assert_eq!(6590, x);
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Questionnaire(u32);

impl Questionnaire {
    fn count(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn merge(self, other: Questionnaire) -> Questionnaire {
        Questionnaire(self.0 | other.0)
    }

    fn from_str(s: &str) -> Self {
        fn apply(s: Questionnaire, c: char) -> Questionnaire {
            s.merge(Questionnaire(1 << (c as u32 - 'a' as u32)))
        }

        s.chars().fold(Questionnaire(0), apply)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::Questionnaire;

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
        let x = EXAMPLE
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(Questionnaire::from_str)
                    .fold(Questionnaire(0), |q1, q2| q1.merge(q2))
                    .count()
            })
            .sum::<usize>();
        eprintln!("{}", x);
    }
}
