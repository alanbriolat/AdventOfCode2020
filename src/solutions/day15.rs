use std::collections::HashMap;

use super::prelude::*;
use crate::util;

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<u64>> {
    let line = util::read_lines(input_path)
        .next()
        .ok_or(crate::Error::Other("no input".into()))?;
    line.split(",")
        .map(|x| x.parse().map_err(crate::Error::from))
        .collect::<Result<Vec<_>, _>>()
}

struct Game {
    turns: u64,
    previous: u64,
    last_seen: HashMap<u64, u64>,
}

impl Game {
    fn new(seed: &[u64]) -> Self {
        assert!(seed.len() > 0);
        Game {
            turns: seed.len() as u64,
            previous: seed.last().cloned().unwrap(),
            last_seen: seed[..seed.len() - 1]
                .iter()
                .enumerate()
                .map(|(i, &x)| (x, (i + 1) as u64))
                .collect(),
        }
    }
}

impl Iterator for Game {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let last_seen = self
            .last_seen
            .get(&self.previous)
            .cloned()
            .unwrap_or(self.turns);
        let next = self.turns - last_seen;
        self.last_seen.insert(self.previous, self.turns);
        self.turns += 1;
        self.previous = next;
        Some(next)
    }
}

/// Part 1: given the `Game` above, find the 2020th number.
///
/// Simply run the game until the 2020th step.
fn part1(input_path: PathBuf) -> crate::Result<String> {
    let seed = read_input(&input_path)?;
    part1_impl(&seed).map(|x| x.to_string())
}

fn part1_impl(seed: &[u64]) -> crate::Result<u64> {
    let mut game = Game::new(seed);
    game.nth(2020 - seed.len() - 1).ok_or("no result".into())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day15part1", || part1(data_path!("day15_input.txt")));
    runner.add("day15part2", || part2(data_path!("day15_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_impl() {
        assert_eq!(part1_impl(&vec![0, 3, 6]).unwrap(), 436);
        assert_eq!(part1_impl(&vec![1, 3, 2]).unwrap(), 1);
        assert_eq!(part1_impl(&vec![2, 1, 3]).unwrap(), 10);
        assert_eq!(part1_impl(&vec![1, 2, 3]).unwrap(), 27);
        assert_eq!(part1_impl(&vec![2, 3, 1]).unwrap(), 78);
        assert_eq!(part1_impl(&vec![3, 2, 1]).unwrap(), 438);
        assert_eq!(part1_impl(&vec![3, 1, 2]).unwrap(), 1836);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day15_input.txt")).unwrap(), "249");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day15_input.txt")).unwrap(), "");
    }
}
