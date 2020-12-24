use std::collections::VecDeque;

use super::prelude::*;
use crate::util;

type Deck = VecDeque<u8>;

/// Run a round of "Combat" between `p1` and `p2` if possible, returning the winner if there is one.
fn play_combat_round<'a>(p1: &'a mut Deck, p2: &'a mut Deck) -> Option<&'a mut Deck> {
    if p1.len() == 0 {
        Some(p2)
    } else if p2.len() == 0 {
        Some(p1)
    } else {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
        None
    }
}

fn score(p: &Deck) -> u64 {
    p.iter()
        .rev()
        .enumerate()
        .map(|(i, &c)| c as u64 * (i as u64 + 1))
        .sum()
}

fn read_input(input_path: &PathBuf) -> crate::Result<(Deck, Deck)> {
    let mut decks: Vec<VecDeque<u8>> = Vec::new();

    for line in util::read_lines(input_path) {
        if line.len() == 0 {
            continue;
        } else if line.starts_with("Player ") {
            decks.push(VecDeque::new());
        } else {
            decks.last_mut().unwrap().push_back(line.parse()?);
        }
    }
    let mut iter = decks.into_iter();
    let p1 = iter.next().unwrap();
    let p2 = iter.next().unwrap();

    Ok((p1, p2))
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let (mut p1, mut p2) = read_input(&input_path)?;

    let winner = loop {
        if let Some(winner) = play_combat_round(&mut p1, &mut p2) {
            break winner;
        }
    };

    Ok(score(&winner).to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day22part1", || part1(data_path!("day22_input.txt")));
    runner.add("day22part2", || part2(data_path!("day22_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day22_example1.txt")).unwrap(), "306");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day22_input.txt")).unwrap(), "32598");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day22_input.txt")).unwrap(), "");
    }
}
