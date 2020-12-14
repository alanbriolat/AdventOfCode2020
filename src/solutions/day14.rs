use std::collections::HashMap;
use std::str::FromStr;

use super::prelude::*;
use crate::util;

type Word = u64;

#[derive(Clone)]
struct Mask {
    pattern: Word,
    value: Word,
}

impl Mask {
    fn identity() -> Self {
        Mask {
            pattern: !0,
            value: 0,
        }
    }

    fn apply(&self, value: Word) -> Word {
        value & self.pattern | self.value
    }
}

impl FromStr for Mask {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pattern: Word = 0;
        let mut value: Word = 0;
        for (i, b) in s.bytes().rev().enumerate() {
            match b {
                b'X' => {
                    pattern |= 1 << i;
                }
                b'0' => {
                    // Nothing to do here, bit is already 0 in both pattern and value
                }
                b'1' => {
                    value |= 1 << i;
                }
                b => return Err(format!("unexpected byte in mask: {}", b).into()),
            }
        }
        Ok(Mask { pattern, value })
    }
}

enum Op {
    Mask(Mask),
    Set(Word, Word),
}

impl FromStr for Op {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = util::str_partition(s, " = ");
        if lhs == "mask" {
            Ok(Op::Mask(rhs.parse()?))
        } else if lhs.starts_with("mem[") {
            Ok(Op::Set(lhs[4..(lhs.len() - 1)].parse()?, rhs.parse()?))
        } else {
            Err(format!("unknown operation: {}", s).into())
        }
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<Op>> {
    util::read_lines(input_path)
        .map(|line| line.parse())
        .collect()
}

struct System {
    mem: HashMap<Word, Word>,
    mask: Mask,
}

impl System {
    fn new() -> Self {
        System {
            mem: HashMap::new(),
            mask: Mask::identity(),
        }
    }

    fn apply(&mut self, op: &Op) {
        match op {
            Op::Mask(mask) => {
                self.mask = mask.clone();
            }
            Op::Set(addr, value) => {
                let value = self.mask.apply(*value);
                self.mem.insert(*addr, value);
            }
        }
    }

    fn run(&mut self, ops: &[Op]) {
        for op in ops {
            self.apply(op);
        }
    }
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let program = read_input(&input_path)?;
    let mut system = System::new();
    system.run(&program);
    let sum: Word = system.mem.values().sum();
    Ok(sum.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day14part1", || part1(data_path!("day14_input.txt")));
    runner.add("day14part2", || part2(data_path!("day14_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day14_example1.txt")).unwrap(), "165");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(data_path!("day14_input.txt")).unwrap(),
            "5055782549997"
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day14_input.txt")).unwrap(), "");
    }
}
