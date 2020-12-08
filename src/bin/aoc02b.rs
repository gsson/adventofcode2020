use std::io::{BufRead, BufReader};

fn main() {
    let f = std::fs::File::open("src/bin/aoc02.txt").unwrap();

    let valid = valid_passwords(BufReader::new(f).lines().map(|l| l.unwrap()));
    eprintln!("{}", valid);
    assert_eq!(747, valid);
}

fn parse_line(line: String) -> (usize, usize, char, String) {
    let mut split = line.split_ascii_whitespace();
    let rule = split.next().unwrap();
    let mut rule = rule.split('-');
    let first = rule.next().unwrap().parse::<usize>().unwrap();
    let second = rule.next().unwrap().parse::<usize>().unwrap();

    let char = split.next().unwrap().chars().next().unwrap();
    let password = split.next().unwrap().into();

    (first - 1, second - 1, char, password)
}

fn password_valid(first: usize, second: usize, char: char, password: &str) -> bool {
    password
        .chars()
        .enumerate()
        .filter(|(i, c)| (*i == first || *i == second) && *c == char)
        .count()
        == 1
}

fn valid_passwords(lines: impl Iterator<Item = String>) -> usize {
    lines
        .map(parse_line)
        .filter(|(first, second, char, password)| password_valid(*first, *second, *char, password))
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

        assert_eq!(1, count);
    }
}
