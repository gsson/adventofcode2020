#![feature(str_split_once)]

use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc16.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let error_rate = Problem::new(&input).error_rate();
    eprintln!("{}", error_rate);
    assert_eq!(28884, error_rate)
}

#[derive(Debug)]
struct FieldRule {
    field: String,
    valid_values: Index,
}

#[derive(Debug)]
struct Index([u128; 8]);

impl Index {
    fn new(ranges: &[ValidRange]) -> Self {
        let mut a = [0u128; 8];
        for range in ranges {
            let (from_i, from_bit) = Self::pos(range.from);
            let (to_i, to_bit) = Self::pos(range.to);
            let from_mask = !(from_bit - 1);
            let to_mask = (to_bit - 1) | to_bit;
            if from_i == to_i {
                a[from_i] |= from_mask & to_mask
            } else {
                a[from_i] |= from_mask;
                a[to_i] |= to_mask;
                for i in from_i + 1..to_i {
                    a[i] |= !0u128;
                }
            }
        }
        Self(a)
    }
    fn pos(v: usize) -> (usize, u128) {
        let i = v >> 7;
        let bit = 1u128 << (v & 0x7f);
        (i, bit)
    }
    fn contains(&self, v: usize) -> bool {
        let (i, bit) = Self::pos(v);
        self.0[i] & bit != 0
    }
}

impl FieldRule {
    fn new(s: &str) -> FieldRule {
        let (field, ranges) = s.split_once(": ").unwrap();
        let valid_values = ranges.split(" or ")
            .map(ValidRange::new)
            .collect::<Vec<_>>();
        let valid_values = Index::new(&valid_values);
        Self {
            field: field.into(),
            valid_values,
        }
    }
}


#[derive(Debug)]
struct ValidRange {
    from: usize,
    to: usize,
}

impl ValidRange {
    fn new(s: &str) -> Self {
        let (from, to) = s.split_once('-').unwrap();
        Self {
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn new(s: &str) -> Self {
        Self(s.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
    }
}

#[derive(Debug)]
struct Problem {
    fields: Vec<FieldRule>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Problem {
    fn new(s: &str) -> Self {
        let (fields, tail) = s.split_once("\n\n").unwrap();
        let (ticket, nearby_tickets) = tail.split_once("\n\n").unwrap();

        let fields: Vec<FieldRule> = fields.split('\n')
            .map(FieldRule::new)
            .collect();

        let ticket = ticket.split('\n')
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(Ticket::new)
            .next()
            .unwrap();
        let nearby_tickets = nearby_tickets.split('\n')
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(Ticket::new)
            .collect();
        Self {
            fields,
            ticket,
            nearby_tickets,
        }
    }

    fn matching_fields(&self, v: usize) -> impl Iterator<Item=&str> + '_ {
        self.fields.iter()
            .filter(move |f| f.valid_values.contains(v))
            .map(|f| f.field.as_str())
    }

    fn error_rate(&self) -> usize {
        self.nearby_tickets.iter()
            .flat_map(|t| t.0.iter())
            .filter(|v| self.matching_fields(**v).next().is_none())
            .sum::<usize>()
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    #[test]
    fn test_error_rate() {
        let problem = Problem::new(EXAMPLE);
        assert_eq!(71, problem.error_rate())
    }
}
