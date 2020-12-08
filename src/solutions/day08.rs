use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::prelude::*;
use crate::util;

type Word = i64;

#[derive(Clone, Copy, Debug)]
enum Op {
    Acc(Word),
    Jmp(Word),
    Nop(Word),
}

impl FromStr for Op {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, operand) = util::str_partition(&s, " ");
        match opcode {
            "acc" => Ok(Op::Acc(operand.parse()?)),
            "jmp" => Ok(Op::Jmp(operand.parse()?)),
            "nop" => Ok(Op::Nop(operand.parse()?)),
            _ => Err(format!("unrecognised operation: {}", s).into()),
        }
    }
}

impl Op {
    fn apply(&self, (pc, acc): (Word, Word)) -> (Word, Word) {
        match self {
            Op::Acc(v) => (pc + 1, acc + v),
            Op::Jmp(v) => (pc + v, acc),
            Op::Nop(_) => (pc + 1, acc),
        }
    }

    fn flip_jmp_nop(&self) -> Option<Self> {
        match self {
            Op::Jmp(v) => Some(Op::Nop(*v)),
            Op::Nop(v) => Some(Op::Jmp(*v)),
            _ => None,
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

    fn get_state(&self) -> (Word, Word) {
        (self.pc, self.acc)
    }

    fn set_state(&mut self, (pc, acc): (Word, Word)) {
        self.pc = pc;
        self.acc = acc;
    }

    fn current_op(&self) -> Option<&Op> {
        if (self.pc as usize) < self.program.len() {
            Some(&self.program[self.pc as usize])
        } else {
            None
        }
    }

    fn current_op_mut(&mut self) -> Option<&mut Op> {
        if (self.pc as usize) < self.program.len() {
            Some(&mut self.program[self.pc as usize])
        } else {
            None
        }
    }

    fn flip_jmp_nop(&mut self) -> bool {
        if let Some(op) = self.current_op_mut() {
            if let Some(new_op) = op.flip_jmp_nop() {
                *op = new_op;
                return true;
            }
        }
        false
    }

    fn step(&mut self) -> Option<(Word, Word)> {
        if let Some(op) = self.current_op() {
            let new_state = op.apply(self.get_state());
            self.set_state(new_state);
            Some(new_state)
        } else {
            None
        }
    }
}

impl Iterator for Machine {
    type Item = (Word, Word);

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
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
    let mut machine = read_input(&input_path)?;
    let initial_state = machine.get_state();

    // Step 1: find the cycle
    let mut history: Vec<(Word, Word)> = vec![initial_state];
    let mut visited: HashMap<Word, usize> = HashMap::new();
    let mut cycle_start: Option<usize> = None;
    for (pc, acc) in std::iter::once(initial_state).chain(&mut machine) {
        if let Some(pos) = visited.get(&pc).cloned() {
            cycle_start = Some(pos);
            break;
        } else {
            visited.insert(pc, history.len());
            history.push((pc, acc));
        }
    }
    let cycle_history = &history[cycle_start.unwrap()..];

    // Step 2: try to break the cycle by flipping 1 op at a time
    'outer: for state in cycle_history {
        machine.set_state(*state);
        // Flip the op if it can be flipped
        if machine.flip_jmp_nop() {
            // Find out if we still have a cycle
            // machine.set_state(initial_state);
            let mut visited: HashSet<Word> = HashSet::new();
            for (pc, _) in &mut machine {
                if !visited.insert(pc) {
                    // Flip it back so we can try with a different instruction
                    machine.set_state(*state);
                    machine.flip_jmp_nop();
                    continue 'outer;
                }
            }
            // If we got this far the program terminated, i.e. the currently flipped op is correct
            break 'outer;
        }
    }

    // Step 3: run the modified program from the initial state to get the final state
    machine.set_state(initial_state);
    let (_, acc) = machine.last().unwrap();
    Ok(acc.to_string())
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
        assert_eq!(part1(data_path!("day08_input.txt")).unwrap(), "2014");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(data_path!("day08_example.txt")).unwrap(), "8");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day08_input.txt")).unwrap(), "2251");
    }
}
