#![feature(test)]
#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::Read;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc07.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let rules = s.lines().map(parse_rule).collect::<Vec<_>>();

    let c = Color::new("shiny gold");
    let contained = find_contained(&c, &rules);
    eprintln!("Contained bags: {}", contained);
    assert_eq!(2976, contained);
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Color(String);

impl Color {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Rule(Color, Vec<(usize, Color)>);

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref A: Regex = Regex::new(r#"(\w+ \w+) bags contain ([^.]+)\."#).unwrap();
        static ref B: Regex = Regex::new(r#"(\d+) (\w+ \w+)"#).unwrap();
    }

    let caps = A.captures(line).unwrap();
    let bag = Color::new(&caps[1]);
    let contains = B
        .captures_iter(&caps[2])
        .map(|cap| (cap[1].parse::<usize>().unwrap(), Color::new(&cap[2])))
        .collect::<Vec<_>>();

    Rule(bag, contains)
}

fn find_contained(outer_color: &Color, rules: &[Rule]) -> usize {
    let mut contains: HashMap<&Color, Vec<(usize, &Color)>> = HashMap::new();
    for rule in rules {
        let v = contains.entry(&rule.0).or_default();
        for r in &rule.1 {
            v.push((r.0, &r.1))
        }
    }

    fn count_contained(
        count: usize,
        outer: &Color,
        contains: &HashMap<&Color, Vec<(usize, &Color)>>,
    ) -> usize {
        let mut acc = 0;
        if let Some(inners) = contains.get(outer) {
            for (inner_count, inner) in inners {
                acc += count_contained(*inner_count, *inner, contains);
            }
        }
        count * (1 + acc)
    }

    count_contained(1, outer_color, &contains) - 1
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::{find_contained, parse_rule, Color};

    const EXAMPLE1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    const EXAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
";

    #[test]
    fn test_bags_example1() {
        let rules = EXAMPLE1.lines().map(parse_rule).collect::<Vec<_>>();

        let c = Color::new("shiny gold");

        assert_eq!(32, find_contained(&c, &rules));
    }

    #[test]
    fn test_bags_example2() {
        let rules = EXAMPLE2.lines().map(parse_rule).collect::<Vec<_>>();

        let c = Color::new("shiny gold");
        assert_eq!(126, find_contained(&c, &rules));
    }
}
