#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc10.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let adapters = input.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (ones, threes) = find_jolt_differential(&adapters);

    eprintln!("{} * {} == {}", ones, threes, ones * threes);
}

fn find_jolt_differential(adapters: &[usize]) -> (usize, usize) {
    let mut adapters = adapters.to_vec();
    adapters.sort_unstable();

    let (_, ones, threes) = adapters.iter()
        .fold((0, 0, 1), |(prev_j, ones, threes), j| {
            match j - prev_j {
                1 => (*j, ones + 1, threes),
                3 => (*j, ones, threes + 1),
                _ => (*j, ones, threes)
            }
        });
    (ones, threes)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4
";
    const EXAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test_jolts_1() {
        let mut adapters = EXAMPLE1.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (ones, threes) = find_jolt_differential(&adapters);

        eprintln!("{} * {} == {}", ones, threes, ones * threes);
        assert_eq!(7, ones);
        assert_eq!(5, threes);
    }

    #[test]
    fn test_jolts_2() {
        let mut adapters = EXAMPLE2.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (ones, threes) = find_jolt_differential(&adapters);

        eprintln!("{} * {} == {}", ones, threes, ones * threes);
        assert_eq!(22, ones);
        assert_eq!(10, threes);
    }
}
