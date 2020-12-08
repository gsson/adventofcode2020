use std::io::{BufRead, BufReader};

fn main() {
    let f = std::fs::File::open("src/bin/aoc01.txt").unwrap();
    let r = BufReader::new(f);
    let mut numbers = r
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let (a, b, c) = find(&mut numbers).unwrap();

    eprintln!("{} * {} * {} = {}", a, b, c, a * b * c);
    assert_eq!(131248694, a * b * c);
}

fn find(numbers: &mut [u32]) -> Option<(u32, u32, u32)> {
    fn find_r(numbers: &[u32]) -> Option<(u32, u32, u32)> {
        if let Some((prefix, tail)) = numbers.split_first() {
            return if let v @ Some(_) = find_o(prefix, tail) {
                v
            } else {
                find_r(tail)
            };
        }
        None
    }

    fn find_o(prefix: &u32, tail: &[u32]) -> Option<(u32, u32, u32)> {
        if let Some((head, tail)) = tail.split_first() {
            if prefix + head >= 2020 {
                return None;
            }
            let needle = 2020 - head - prefix;

            if let Ok(n) = tail.binary_search(&needle) {
                return Some((*prefix, *head, tail[n]));
            }
            return find_o(prefix, tail);
        }
        None
    }

    numbers.sort_unstable();

    find_r(numbers)
}

#[cfg(test)]
mod tests {
    use crate::find;

    #[test]
    fn it_works() {
        let mut v = [1721, 979, 366, 299, 675, 1456];
        let (a, b, c) = find(&mut v).unwrap();
        assert_eq!(241861950, a * b * c);
    }
}
