#![feature(test)]
#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc04.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let passports = parse(&s);

    eprintln!("Valid passports: {:#?}", passports);
    assert_eq!(226, passports);
}

const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse(s: &str) -> usize {
    let mut valid_passports = 0;
    let mut fields: HashMap<&str, &str> = HashMap::new();
    for line in s.lines() {
        if line.is_empty() {
            if REQUIRED.iter().all(|s| fields.contains_key(s)) {
                valid_passports += 1;
            }
            fields.clear();
        }
        for item in line.split_ascii_whitespace() {
            if let Some((k, v)) = item.split_once(':') {
                assert!(!v.is_empty());
                fields.insert(k, v);
            }
        }
    }
    if !fields.is_empty() {
        if REQUIRED.iter().all(|s| fields.contains_key(s)) {
            valid_passports += 1;
        }
        fields.clear();
    }

    valid_passports
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::parse;

    const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    #[test]
    fn it_works() {
        let passports = parse(EXAMPLE);

        assert_eq!(2, passports);
    }
}
