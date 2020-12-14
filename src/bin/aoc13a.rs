#![feature(test)]
#![feature(str_split_once)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc13.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let (t, buses) = input.split_once('\n').unwrap();
    let t = t.parse::<usize>().unwrap();
    let mut bus_id = 0;
    let mut t2 = usize::max_value();
    for b in buses.trim_end().split(',') {
        if b == "x" {
            continue
        }

        let b = b.parse::<usize>().unwrap();
        let c = ((t + b) / b) * b - t;

        if c < t2 {
            t2 = c;
            bus_id = b;
        }
    }
    eprintln!("{} * {} == {}", bus_id, t2, bus_id * t2);

}


#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_simulation() {
        let (t, buses) = EXAMPLE.split_once('\n').unwrap();
        let t = t.parse::<usize>().unwrap();
        let mut bus_id = 0;
        let mut t2 = usize::max_value();
        for b in buses.split(',') {
            if b == "x" {
                continue
            }

            let b = b.parse::<usize>().unwrap();
            let c = ((t + b) / b) * b - t;

            if c < t2 {
                t2 = c;
                bus_id = b;
            }
        }
        eprintln!("{} * {} == {}", bus_id, t2, bus_id * t2);
        assert_eq!(295, bus_id * t2);
    }

}
