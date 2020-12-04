use std::collections::{HashMap, HashSet};

use super::prelude::*;
use crate::util;

#[allow(dead_code)]
const EXPECTED_FIELDS: &'static [&'static str] =
    &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

const REQUIRED_FIELDS: &'static [&'static str] = &[
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
    // "cid",
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
    Err("unimplemented".into())
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
        assert_eq!(part2(data_path!("day04input.txt")).unwrap(), "");
    }
}
