#![feature(str_split_once)]
#![feature(iterator_fold_self)]

use std::io::Read;
use std::fmt::Debug;
use adventofcode2020::BitIndex;
use std::ops::RangeInclusive;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc16.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();

    let problem = Problem::new(&input);
    let fields = problem.map_fields();
    let o = fields.iter()
        .filter_map(|(a, b)| if a.starts_with("departure") { Some(b) } else { None })
        .product::<usize>();
    fields.iter()
        .for_each(|(f, v)| eprintln!("{}: {}", f, v));
    eprintln!("Departure field product: {:?}", o);
    assert_eq!(1001849322119, o);
}

type ValueIndex = BitIndex<8>;
type FieldIndex = BitIndex<1>;

#[derive(Clone, Debug)]
struct FieldRule {
    id: usize,
    field: String,
    valid_values: ValueIndex,
}


impl FieldRule {
    fn new(id: usize, s: &str) -> FieldRule {
        fn range(s: &str) -> RangeInclusive<usize> {
            let (from, to) = s.split_once('-').unwrap();
            RangeInclusive::new(from.parse().unwrap(), to.parse().unwrap())
        }

        let (field, ranges) = s.split_once(": ").unwrap();
        let valid_values = ranges.split(" or ")
            .map(range);
        let valid_values = ValueIndex::from_ranges(valid_values);
        Self {
            id,
            field: field.into(),
            valid_values,
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
            .enumerate()
            .map(|(i, s)|FieldRule::new(i, s))
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

    fn matching_fields(&self, v: usize) -> impl Iterator<Item=&FieldRule> + '_ {
        self.fields.iter()
            .filter(move |f| f.valid_values.contains(v))
    }

    fn valid_ticket(&self, ticket: &Ticket) -> bool {
        ticket.0.iter()
            .all(|v| self.matching_fields(*v).next().is_some())
    }

    fn matching_field_index(&self, v: usize) -> FieldIndex {
        self.matching_fields(v)
            .fold(FieldIndex::empty(), |mut i, f| { i += f.id; i})
    }

    fn map_fields(&self) -> Vec<(&str, usize)> {
        let valid_tickets = self.nearby_tickets.iter()
            .filter(|t| self.valid_ticket(*t))
            .collect::<Vec<_>>();

        let valid_tickets = valid_tickets.into_iter()
            .map(|t| t.0.iter().map(|v| self.matching_field_index(*v)).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let ticket_fields = self.ticket.0.len();
        let possible_fields = self.fields.len();
        let mut candidates = vec![FieldIndex::from_range(..possible_fields); ticket_fields];

        for t in &valid_tickets {
            for i in 0..ticket_fields {
                candidates[i] &= &t[i]
            }
        }

        while candidates.iter().any(|f| f.len() > 1) {
            let (single, multi): (Vec<&mut FieldIndex>, Vec<&mut FieldIndex>) = candidates.iter_mut()
                .partition(|c| c.len() == 1);

            let assigned = single.iter()
                .fold(FieldIndex::empty(), |mut a, b| {
                    a |= &**b;
                    a
                });

            for t in multi {
                *t -= &assigned;
            }
        }

        candidates.iter()
            .enumerate()
            .map(move |(i, j)| {
                let j = j.iter().next().unwrap();
                (self.fields[j].field.as_str(), self.ticket.0[i])
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn test_valid_tickets() {
        let problem = Problem::new(EXAMPLE);
        eprintln!("{:?}", problem.map_fields());
    }
}
