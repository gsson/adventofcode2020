#![feature(test)]
#![feature(str_split_once)]
#![feature(map_first_last)]

use std::collections::BTreeMap;
use std::io::Read;
use std::collections::btree_map::Entry;
use std::cmp::Ordering;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc09.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let numbers = input.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let weakness = find_encryption_weakness(25, &numbers);
    eprintln!("Weakness: {}", weakness);
    assert_eq!(268878261, weakness);
}

fn find_encryption_weakness(window_size: usize, numbers: &[usize]) -> usize {
    let (end, v) = find_first_invalid(window_size, &numbers);
    for i in (1..end).rev() {
        for j in (0..i).rev() {
            let slice = &numbers[j..i];
            let sum = slice.iter().sum::<usize>();
            match v.cmp(&sum) {
                Ordering::Less => break,
                Ordering::Equal => return slice.iter().min().unwrap() + slice.iter().max().unwrap(),
                Ordering::Greater => ()
            }
        }
    }
    unreachable!()
}

fn find_first_invalid(window_size: usize, numbers: &[usize]) -> (usize, usize) {
    let mut v: BTreeMap<usize, usize> = BTreeMap::new();
    numbers[0..window_size].iter()
        .for_each(|n| *v.entry(*n).or_default() += 1);

    for i in window_size .. numbers.len() {
        let window_start = i - window_size;

        let next = numbers[i];

        if !is_valid(next, &v) {
            return (i, next);
        }

        *v.entry(next)
            .or_default() += 1;
        match v.entry(numbers[window_start]) {
            Entry::Occupied(o) if *o.get() == 1 => { o.remove(); },
            Entry::Occupied(o) => *o.into_mut() -= 1,
            _ => unreachable!()
        }
    }
    unreachable!()
}

fn is_valid(n: usize, v: &BTreeMap<usize, usize>) -> bool {
    for k in v.keys() {
        if n < *k {
            return false;
        }
        if v.contains_key(&(n - k)) {
            return true
        }
    }
    false
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::bench::Bencher;

    use crate::*;

    const EXAMPLE: (usize, &str) = (5, "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
");

    #[test]
    fn test_xmas() {
        let (window_size, s) = &EXAMPLE;
        let window_size = *window_size;
        let numbers = s.lines()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(62, find_encryption_weakness(window_size, &numbers));
    }
}
