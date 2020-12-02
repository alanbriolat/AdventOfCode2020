use std::path::PathBuf;
use std::str::FromStr;

use crate::util;

#[derive(Debug)]
struct Policy {
    min: u8,
    max: u8,
    char: char,
}

impl Policy {
    fn valid_by_char_count(&self, password: &str) -> bool {
        let mut count: u8 = 0;
        for c in password.chars() {
            if c == self.char {
                count += 1;
            }
        }
        count >= self.min && count <= self.max
    }

    fn valid_by_char_position(&self, password: &str) -> bool {
        let match_min = password.as_bytes()[(self.min - 1) as usize] as char == self.char;
        let match_max = password.as_bytes()[(self.max - 1) as usize] as char == self.char;
        (match_min || match_max) && !(match_min && match_max)
    }
}

impl FromStr for Policy {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min_max, char) = util::str_partition(s, " ");
        let (min, max) = util::str_partition(min_max, "-");
        Ok(Policy {
            min: min.parse()?,
            max: max.parse()?,
            char: char.parse()?,
        })
    }
}

struct Example {
    policy: Policy,
    password: String,
}

impl FromStr for Example {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_policy, password) = util::str_partition(s, ": ");
        Ok(Example {
            policy: raw_policy.parse()?,
            password: password.into(),
        })
    }
}

fn read_input(input_path: PathBuf) -> crate::Result<Vec<Example>> {
    util::read_lines(&input_path)
        .map(|line| line.parse())
        .collect()
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(input_path)?;
    let valid_count = data
        .iter()
        .filter(|example| example.policy.valid_by_char_count(&example.password))
        .count();
    Ok(valid_count.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let data = read_input(input_path)?;
    let valid_count = data
        .iter()
        .filter(|example| example.policy.valid_by_char_position(&example.password))
        .count();
    Ok(valid_count.to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day02part1", || part1(data_path!("day02_input.txt")));
    runner.add("day02part2", || part2(data_path!("day02_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day02_input.txt")).unwrap(), "454");
    }

    #[test]
    fn test_part2_validation_a() {
        let example: Example = "1-3 a: abcdef".parse().unwrap();
        assert!(example.policy.valid_by_char_position(&example.password));
    }

    #[test]
    fn test_part2_validation_b() {
        let example: Example = "1-3 b: cdefg".parse().unwrap();
        assert!(!example.policy.valid_by_char_position(&example.password));
    }

    #[test]
    fn test_part2_validation_c() {
        let example: Example = "2-9 c: ccccccccc".parse().unwrap();
        assert!(!example.policy.valid_by_char_position(&example.password));
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day02_input.txt")).unwrap(), "649");
    }
}
