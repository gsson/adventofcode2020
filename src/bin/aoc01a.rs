use std::io::{BufRead, BufReader};

fn main() {
    let f = std::fs::File::open("src/bin/aoc01.txt").unwrap();
    let r = BufReader::new(f);

    let mut numbers = r
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let (a, b) = find(&mut numbers).unwrap();

    eprintln!("{} * {} = {}", a, b, a * b);
    assert_eq!(997899, a * b);
}

fn find(numbers: &mut [u32]) -> Option<(u32, u32)> {
    fn find_r(head: &u32, tail: &[u32]) -> Option<(u32, u32)> {
        let needle = 2020 - head;

        if let Ok(n) = tail.binary_search(&needle) {
            return Some((*head, tail[n]));
        }
        if let Some((head, tail)) = tail.split_first() {
            return find_r(head, tail);
        }
        None
    }

    numbers.sort_unstable();

    if let Some((head, tail)) = numbers.split_first() {
        return find_r(head, tail);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::find;

    #[test]
    fn it_works() {
        let mut v = [1721, 979, 366, 299, 675, 1456];
        let (a, b) = find(&mut v).unwrap();
        assert_eq!(514579, a * b);
    }
}
