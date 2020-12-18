use std::collections::BTreeMap;
use std::io::Read;
use std::collections::btree_map::Entry;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc09.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let numbers = input.lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let (_, solution) = find_first_invalid(25, &numbers);
    eprintln!("{}", solution);
    assert_eq!(1930745883, solution);
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

        assert_eq!(127, find_first_invalid(window_size, &numbers).1);
    }
}
