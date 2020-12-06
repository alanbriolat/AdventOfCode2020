use std::collections::HashSet;

use super::prelude::*;
use crate::util;

type Question = u8;

struct Person(HashSet<Question>);

struct Group(Vec<Person>);

impl Group {
    fn question_union(&self) -> HashSet<Question> {
        let mut iter = self.0.iter().map(|p| &p.0);
        let mut acc = iter.next().cloned().unwrap_or(HashSet::new());
        for other in iter {
            for &q in other {
                acc.insert(q);
            }
        }
        acc
    }

    fn question_intersection(&self) -> HashSet<Question> {
        let mut iter = self.0.iter().map(|p| &p.0);
        let mut acc = iter.next().cloned().unwrap_or(HashSet::new());
        for other in iter {
            acc.retain(|q| other.contains(q));
        }
        acc
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<Group>> {
    let mut output: Vec<Group> = Vec::new();
    let mut current: Vec<Person> = Vec::new();

    fn finalise(current: &mut Vec<Person>, output: &mut Vec<Group>) {
        if current.len() > 0 {
            output.push(Group(current.drain(..).collect()));
        }
    }

    for line in util::read_lines(input_path) {
        if line.len() == 0 {
            finalise(&mut current, &mut output);
        } else {
            current.push(Person(line.bytes().collect()));
        }
    }
    finalise(&mut current, &mut output);

    Ok(output)
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    Ok(read_input(&input_path)?
        .iter()
        .map(|group| group.question_union().len())
        .sum::<usize>()
        .to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Ok(read_input(&input_path)?
        .iter()
        .map(|group| group.question_intersection().len())
        .sum::<usize>()
        .to_string())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day06part1", || part1(data_path!("day06_input.txt")));
    runner.add("day06part2", || part2(data_path!("day06_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day06_example.txt")).unwrap(), "11");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day06_input.txt")).unwrap(), "6551");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day06_input.txt")).unwrap(), "3358");
    }
}
