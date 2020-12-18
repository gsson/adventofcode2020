use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc15.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let mut game = Game::new(&input);
    let next = game.until(2020);

    eprintln!("Next: {}", next);
    assert_eq!(441, next);
}

#[derive(Default)]
struct Game {
    map: HashMap<usize, usize>,
    last: usize,
    turn: usize,
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.map.insert(self.last, self.turn) {
            None => 0,
            Some(j) => self.turn - j,
        };
        self.turn += 1;
        self.last = next;

        Some(next)
    }
}

impl Game {
    fn until(&mut self, turn: usize) -> usize {
        assert!(turn > self.turn);
        self.nth(turn - self.turn - 1).unwrap()
    }

    fn new(input: &str) -> Self {
        let mut init = Self::default();

        input.split(',').enumerate()
            .map(|(turn, v)| (turn + 1, v.parse::<usize>().unwrap()))
            .for_each(|(turn, v)| {
                init.map.insert(v, turn);
                init.last = v;
                init.turn = turn;
            });
        init
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLES: [(usize, &str); 7] = [
        (436, "0,3,6"),
        (1, "1,3,2"),
        (10, "2,1,3"),
        (27, "1,2,3"),
        (78, "2,3,1"),
        (438, "3,2,1"),
        (1836, "3,1,2"),
    ];

    #[test]
    fn test() {
        for (expected, input) in &EXAMPLES {
            let mut game = Game::new(*input);
            assert_eq!(*expected, game.until(2020));
        }
    }

    #[test]
    fn test_next() {
        let game = Game::new("0,3,6");
        assert_eq!(vec![0usize, 3, 3, 1, 0, 4, 0], game.take(7).collect::<Vec<_>>());
    }
}
