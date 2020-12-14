#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc13.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let solution = solve(&input);
    eprintln!("{}", solution);
    assert_eq!(1001569619313439, solution);
}

fn parse(input: &str) -> impl Iterator<Item=(isize, isize)> + '_ {
    let (_, buses) = input.split_once('\n').unwrap();
    buses.trim_end().split(',')
        .enumerate()
        .filter_map(|(i, b)| if b == "x" { None } else { Some((b.parse::<isize>().unwrap(), i as isize)) })
}

fn next_ts(ts: isize, step: usize, mod_shift: isize, modulo: isize) -> isize {
    (ts ..).step_by(step)
        .find(|v| (v + mod_shift) % modulo == 0)
        .unwrap()
}

fn solve(buses: &str) -> isize {
    let mut buses = parse(buses);
    let (first_bus_id, first_offset) = buses.next().unwrap();
    let (ts, _) = buses
        .fold((0, first_bus_id), |(ts, n), (bus_id, offset)| (next_ts(ts, n as usize, offset - first_offset, bus_id), n * bus_id));
    ts - first_offset
}



#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLES: [(isize, &str); 6] = [
        (1068781, "939
7,13,x,x,59,x,31,19"),
        (3417, "0
17,x,13,19"),
        (754018, "0
67,7,59,61"),
        (779210, "0
67,x,7,59,61"),
        (1261476, "0
67,7,x,59,61"),
        (1202161486, "0
1789,37,47,1889")
    ];

    #[test]
    fn test_simulation() {
        for (expected, buses) in &EXAMPLES {
            let ts = solve(*buses);
            eprintln!("{}", ts);
            assert_eq!(*expected, ts);
        }
    }


    #[bench]
    fn benchmark(bencher: &mut Bencher) {
        let mut f = std::fs::File::open("src/bin/aoc13.txt").unwrap();
        let mut input = String::new();
        f.read_to_string(&mut input).unwrap();
        bencher.iter(move || solve(&input));
    }
}
