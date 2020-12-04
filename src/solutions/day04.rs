use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use super::prelude::*;
use crate::util;

#[allow(dead_code)]
const EXPECTED_FIELDS: &'static [&'static str] =
    &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

const REQUIRED_FIELDS: &'static [&'static str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
    // "cid",
];

type Validator = fn(&String) -> bool;

fn validate_range<I: FromStr + PartialOrd>(v: &str, range: RangeInclusive<I>) -> bool {
    match v.parse::<I>() {
        Ok(year) => range.contains(&year),
        Err(_) => false,
    }
}

fn validate_byr(v: &String) -> bool {
    validate_range::<u16>(v, 1920..=2002)
}

fn validate_iyr(v: &String) -> bool {
    validate_range::<u16>(v, 2010..=2020)
}

fn validate_eyr(v: &String) -> bool {
    validate_range::<u16>(v, 2020..=2030)
}

fn validate_hgt(v: &String) -> bool {
    let (amount, unit) = v.split_at(v.len() - 2);
    let range = match unit {
        "cm" => 150..=193,
        "in" => 59..=76,
        _ => return false,
    };
    validate_range::<u8>(amount, range)
}

fn validate_hex_char(b: &u8) -> bool {
    (b'0'..=b'9').contains(b) || (b'a'..=b'f').contains(b)
}

fn validate_hcl(v: &String) -> bool {
    let v = v.as_bytes();
    v.len() == 7 && v[0] == b'#' && v[1..].iter().all(validate_hex_char)
}

fn validate_ecl(v: &String) -> bool {
    match v.as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(v: &String) -> bool {
    let v = v.as_bytes();
    let range = b'0'..=b'9';
    v.len() == 9 && v.iter().all(|b| range.contains(b))
}

const VALIDATORS: &'static [(&'static str, Validator)] = &[
    ("byr", validate_byr as Validator),
    ("iyr", validate_iyr as Validator),
    ("eyr", validate_eyr as Validator),
    ("hgt", validate_hgt as Validator),
    ("hcl", validate_hcl as Validator),
    ("ecl", validate_ecl as Validator),
    ("pid", validate_pid as Validator),
];

#[derive(Debug)]
struct Passport {
    data: HashMap<String, String>,
}

impl Passport {
    fn has_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|k| self.data.contains_key(k.to_owned()))
    }

    fn is_valid(&self) -> bool {
        VALIDATORS
            .iter()
            .all(|(field, validator)| match self.data.get(field.to_owned()) {
                Some(value) => validator(value),
                None => false,
            })
    }
}

fn read_input(input_path: PathBuf) -> crate::Result<Vec<Passport>> {
    let mut output: Vec<Passport> = Vec::new();
    let mut current: Vec<(String, String)> = Vec::new();

    fn finalise(current: &mut Vec<(String, String)>, output: &mut Vec<Passport>) {
        if current.len() > 0 {
            let passport = Passport {
                data: current.drain(0..current.len()).collect(),
            };
            output.push(passport);
        }
    };

    for line in util::read_lines(&input_path) {
        if line.len() == 0 {
            finalise(&mut current, &mut output);
        } else {
            for item in line.split(" ") {
                let (field, value) = util::str_partition(item, ":");
                current.push((field.into(), value.into()));
            }
        }
    }
    finalise(&mut current, &mut output);

    Ok(output)
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let passports = read_input(input_path)?;
    let valid_count = passports
        .iter()
        .filter(|&p| p.has_required_fields())
        .count();
    Ok(valid_count.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let passports = read_input(input_path)?;
    let valid_count = passports.iter().filter(|&p| p.is_valid()).count();
    Ok(valid_count.to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day04part1", || part1(data_path!("day04_input.txt")));
    runner.add("day04part2", || part2(data_path!("day04_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day04_example.txt")).unwrap(), "2");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day04_input.txt")).unwrap(), "182");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day04_input.txt")).unwrap(), "109");
    }
}
