use std::str::FromStr;

use num;
use num_derive::FromPrimitive;

use super::prelude::*;
use crate::util::{self, Vector2D};

type Position = Vector2D<i64>;

#[derive(Clone, Copy, Debug, FromPrimitive)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn rotate(self, amount: i64) -> Self {
        let mut new = ((self as i64) + amount) % 4;
        if new < 0 {
            new += 4;
        }
        num::FromPrimitive::from_i64(new).unwrap()
    }
}

impl Into<Vector2D<i64>> for Direction {
    fn into(self) -> Position {
        match self {
            Direction::North => Vector2D(0, -1),
            Direction::East => Vector2D(1, 0),
            Direction::South => Vector2D(0, 1),
            Direction::West => Vector2D(-1, 0),
        }
    }
}

enum Rotation {
    Left,
    Right,
}

enum Action {
    Translate(Direction, i64),
    Rotate(Rotation, i64),
    Forward(i64),
}

impl FromStr for Action {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, raw_value) = s.split_at(1);
        let value: i64 = raw_value.parse()?;
        match action {
            "N" => Ok(Action::Translate(Direction::North, value)),
            "E" => Ok(Action::Translate(Direction::East, value)),
            "S" => Ok(Action::Translate(Direction::South, value)),
            "W" => Ok(Action::Translate(Direction::West, value)),
            "L" => Ok(Action::Rotate(Rotation::Left, value)),
            "R" => Ok(Action::Rotate(Rotation::Right, value)),
            "F" => Ok(Action::Forward(value)),
            a => Err(format!("unrecognised action: {}", a).into()),
        }
    }
}

struct State {
    position: Position,
    facing: Direction,
}

impl Default for State {
    fn default() -> Self {
        State {
            position: Vector2D(0, 0),
            facing: Direction::East,
        }
    }
}

impl State {
    fn apply(&mut self, action: Action) {
        match action {
            Action::Translate(direction, value) => {
                let direction: Position = direction.into();
                let change = direction * value;
                self.position = self.position + change;
            },
            Action::Rotate(direction, value) => {
                let multiplier = match direction {
                    Rotation::Left => -1,
                    Rotation::Right => 1,
                };
                self.facing = self.facing.rotate((value / 90) * multiplier);
            },
            Action::Forward(value) => {
                let direction: Position = self.facing.into();
                let change = direction * value;
                self.position = self.position + change;
            },
        }
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<Action>> {
    util::read_lines(input_path)
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let actions = read_input(&input_path)?;
    let mut state = State::default();
    for action in actions {
        state.apply(action);
    }
    Ok(state.position.manhattan_length().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day12part1", || part1(data_path!("day12_input.txt")));
    runner.add("day12part2", || part2(data_path!("day12_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day12_example.txt")).unwrap(), "");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day12_input.txt")).unwrap(), "");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day12_input.txt")).unwrap(), "");
    }
}