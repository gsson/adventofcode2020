use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    let f = std::fs::File::open("src/bin/aoc02.txt").unwrap();

    let valid = valid_passwords(BufReader::new(f).lines().map(|l| l.unwrap()));
    eprintln!("Valid passwords: {}", valid);
    assert_eq!(424, valid);
}

fn parse_line(line: String) -> (RangeInclusive<usize>, char, String) {
    let mut split = line.split_ascii_whitespace();
    let rule = split.next().unwrap();
    let mut rule = rule.split('-');
    let low = rule.next().unwrap().parse::<usize>().unwrap();
    let high = rule.next().unwrap().parse::<usize>().unwrap();

    let char = split.next().unwrap().chars().next().unwrap();
    let password = split.next().unwrap().into();

    (low..=high, char, password)
}

fn password_valid(repeat: &RangeInclusive<usize>, char: char, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == char).count();
    repeat.contains(&count)
}

fn valid_passwords(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(parse_line)
        .filter(|(repeat, char, password)| password_valid(repeat, *char, password))
        .count()
}

#[cfg(test)]
mod tests {
    use crate::valid_passwords;
    const EXAMPLE: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn it_works() {
        let count = valid_passwords(EXAMPLE.lines().map(|s| s.into()));

        assert_eq!(2, count);
    }
}
