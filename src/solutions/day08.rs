use std::str::FromStr;

use super::prelude::*;
use crate::util;

type Word = i64;

enum Op {
    Acc(Word),
    Jmp(Word),
    Nop,
}

impl FromStr for Op {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, operand) = util::str_partition(&s, " ");
        match opcode {
            "acc" => Ok(Op::Acc(operand.parse()?)),
            "jmp" => Ok(Op::Jmp(operand.parse()?)),
            "nop" => Ok(Op::Nop),
            _ => Err(format!("unrecognised operation: {}", s).into()),
        }
    }
}

type Program = Vec<Op>;

struct Machine {
    program: Program,
    pc: Word,
    acc: Word,
}

impl Machine {
    fn new(program: Program) -> Self {
        Machine {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        let jump = match self.program[self.pc as usize] {
            Op::Acc(v) => {
                self.acc += v;
                1
            }
            Op::Jmp(v) => v,
            Op::Nop => 1,
        };
        self.pc += jump;
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Machine> {
    let program = util::read_lines(input_path)
        .map(|line| line.parse::<Op>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Machine::new(program))
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let mut machine = read_input(&input_path)?;
    let mut visited: Vec<u8> = Vec::with_capacity(machine.program.len());
    visited.resize(machine.program.len(), 0);
    while visited[machine.pc as usize] == 0 {
        visited[machine.pc as usize] += 1;
        machine.step();
    }
    Ok(machine.acc.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day08part1", || part1(data_path!("day08_input.txt")));
    runner.add("day08part2", || part2(data_path!("day08_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day08_example.txt")).unwrap(), "5");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day08_input.txt")).unwrap(), "");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day08_input.txt")).unwrap(), "");
    }
}
