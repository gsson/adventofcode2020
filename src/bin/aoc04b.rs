#![feature(test)]
#![feature(str_split_once)]

use std::collections::HashMap;
use std::io::Read;
use std::ops::RangeInclusive;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc04.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let (valid, invalid) = parse(&s);

    eprintln!("Valid: {}", valid);
    eprintln!("Invalid: {}", invalid);
    assert_eq!(160, valid)
}

fn valid_year(s: &str, valid_range: RangeInclusive<usize>) -> Option<String> {
    if s.len() != 4 {
        return None;
    }
    match s.parse::<usize>() {
        Ok(year) if valid_range.contains(&year) => Some(s.to_string()),
        _ => None,
    }
}

fn valid_length(
    s: &str,
    valid_cm_range: RangeInclusive<usize>,
    valid_in_range: RangeInclusive<usize>,
) -> Option<String> {
    if s.ends_with("cm") {
        match s[0..s.len() - 2].parse::<usize>() {
            Ok(l) if valid_cm_range.contains(&l) => Some(s.to_string()),
            _ => None,
        }
    } else if s.ends_with("in") {
        match s[0..s.len() - 2].parse::<usize>() {
            Ok(l) if valid_in_range.contains(&l) => Some(s.to_string()),
            _ => None,
        }
    } else {
        None
    }
}

fn valid_hex_color(s: &str) -> Option<String> {
    if s.len() != 7 {
        return None;
    }
    let mut chars = s.chars();
    if Some('#') != chars.next() {
        return None;
    }
    for c in chars {
        match c {
            '0'..='9' | 'a'..='f' => {}
            _ => return None,
        }
    }
    Some(s.to_string())
}

fn valid_named_color(s: &str) -> Option<String> {
    match s {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => Some(s.to_string()),
        _ => None,
    }
}

fn valid_passport_id(s: &str) -> Option<String> {
    if s.len() != 9 {
        return None;
    }
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(s.to_string())
}

fn validate_passport(fields: &HashMap<&str, &str>) -> Option<()> {
    fields
        .get("byr")
        .and_then(|v| valid_year(*v, 1920..=2002))?;
    fields
        .get("iyr")
        .and_then(|v| valid_year(*v, 2010..=2020))?;
    fields
        .get("eyr")
        .and_then(|v| valid_year(*v, 2020..=2030))?;

    fields
        .get("hgt")
        .and_then(|v| valid_length(*v, 150..=193, 59..=76))?;

    fields.get("hcl").and_then(|v| valid_hex_color(*v))?;
    fields.get("ecl").and_then(|v| valid_named_color(*v))?;
    fields.get("pid").and_then(|v| valid_passport_id(*v))?;
    Some(())
}

fn parse_line(line: &str) -> impl Iterator<Item = (&str, &str)> {
    line.split_ascii_whitespace()
        .filter_map(|item| item.split_once(':'))
}

fn parse(s: &str) -> (usize, usize) {
    let mut valid_passports = 0;
    let mut invalid_passports = 0;
    let mut fields: HashMap<&str, &str> = HashMap::new();
    for line in s.lines() {
        if line.is_empty() {
            if validate_passport(&fields).is_some() {
                valid_passports += 1;
            } else {
                invalid_passports += 1;
            }
            fields.clear();
        }
        parse_line(line).for_each(|(k, v)| {
            fields.insert(k, v);
        });
    }
    if !fields.is_empty() {
        if validate_passport(&fields).is_some() {
            valid_passports += 1;
        } else {
            invalid_passports += 1;
        }
    }

    (valid_passports, invalid_passports)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::parse;

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    #[test]
    fn it_works() {
        assert_eq!((4, 0), parse(VALID));
        assert_eq!((0, 4), parse(INVALID));
    }
}
