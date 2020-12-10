#![feature(test)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc10.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let adapters = input.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let combinations = find_adapter_combinations(&adapters);

    eprintln!("Combinations: {}", combinations);
    assert_eq!(1322306994176, combinations);
}

fn find_adapter_combinations(adapters: &[usize]) -> usize {
    let mut adapters = adapters.to_vec();
    adapters.sort_unstable();
    adapters.push(adapters.last().unwrap() + 3);

    let mut memo: Vec<Option<usize>> = vec![None; adapters.len()];
    memo[adapters.len() - 1] = Some(1);
    fn combinations(in_jolt: usize, adapters: &[usize], memo: &mut[Option<usize>]) -> usize {
        if let Some(n) = memo.first().unwrap() {
            *n
        } else {
            match adapters {
                [head1, head2, head3, ..] if head3 - in_jolt <= 3 => {
                    let c =
                        combinations(*head1, &adapters[1..], &mut memo[1..]) +
                            combinations(*head2, &adapters[2..], &mut memo[2..]) +
                            combinations(*head3, &adapters[3..], &mut memo[3..]);
                    memo[0] = Some(c);
                    c
                }
                [head1, head2, ..] if head2 - in_jolt <= 3 => {
                    let c =
                        combinations(*head1, &adapters[1..], &mut memo[1..]) +
                            combinations(*head2, &adapters[2..], &mut memo[2..]);
                    memo[0] = Some(c);
                    c
                }
                [head1, ..] if head1 - in_jolt <= 3 => {
                    let c =
                        combinations(*head1, &adapters[1..], &mut memo[1..]);
                    memo[0] = Some(c);
                    c
                }
                _ => unreachable!()
            }
        }
    }

    combinations(0, &adapters, &mut memo)
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

        let combinations = find_adapter_combinations(&adapters);

        eprintln!("Combinations: {}", combinations);
        assert_eq!(8, combinations);
    }

    #[test]
    fn test_jolts_2() {
        let mut adapters = EXAMPLE2.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let combinations = find_adapter_combinations(&adapters);

        eprintln!("Combinations: {}", combinations);
        assert_eq!(19208, combinations);
    }

    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc10.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();

        let adapters = input.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        bencher.iter(|| find_adapter_combinations(&adapters));
    }
}
