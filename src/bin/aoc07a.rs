#![feature(test)]
#![feature(str_split_once)]

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc07.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let rules = s.lines().map(parse_rule).collect::<Vec<_>>();

    let c = Color("shiny gold".to_string());
    let containers = find_containers(&c, &rules);
    eprintln!("Containing bags: {}", containers);

    assert_eq!(246, containers);
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Color(String);

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Rule(Color, Vec<(usize, Color)>);

fn parse_rule(line: &str) -> Rule {
    lazy_static! {
        static ref A: Regex = Regex::new(r#"(\w+ \w+) bags contain ([^.]+)\."#).unwrap();
        static ref B: Regex = Regex::new(r#"(\d+) (\w+ \w+)"#).unwrap();
    }

    let caps = A.captures(line).unwrap();
    let bag = Color(caps[1].to_string());
    let contains = B
        .captures_iter(&caps[2])
        .map(|cap| (cap[1].parse::<usize>().unwrap(), Color(cap[2].to_string())))
        .collect::<Vec<_>>();

    Rule(bag, contains)
}

fn find_containers(inner_color: &Color, rules: &[Rule]) -> usize {
    let mut contained_by: HashMap<&Color, Vec<&Color>> = HashMap::new();
    for rule in rules {
        for (_, c) in &rule.1 {
            contained_by.entry(c).or_default().push(&rule.0)
        }
    }

    let mut queue = VecDeque::new();
    let mut candidates: HashSet<&Color> = HashSet::new();
    queue.push_back(inner_color);
    candidates.insert(inner_color);

    while let Some(next) = queue.pop_back() {
        if let Some(new_candidates) = contained_by.get(next) {
            for c in new_candidates {
                if candidates.insert(*c) {
                    queue.push_back(*c);
                }
            }
        }
    }
    candidates.len() - 1
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::{find_containers, parse_rule, Color};

    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    #[test]
    fn test_seat() {
        let rules = EXAMPLE.lines().map(parse_rule).collect::<Vec<_>>();

        let c = Color("shiny gold".to_string());
        assert_eq!(4, find_containers(&c, &rules));
    }
}
