use std::str::FromStr;

use super::prelude::*;
use crate::util;
use crate::vector::{Vector, Vector2D};

fn rotate_vector(v: Vector2D<i64>, mut steps: i64) -> Vector2D<i64> {
    steps %= 4;
    if steps < 0 {
        steps += 4;
    }
    match steps {
        0 => v,
        1 => Vector([-v[1], v[0]]),
        2 => Vector([-v[0], -v[1]]),
        3 => Vector([v[1], -v[0]]),
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Into<Vector2D<i64>> for Direction {
    fn into(self) -> Vector2D<i64> {
        match self {
            Direction::North => Vector([0, -1]),
            Direction::East => Vector([1, 0]),
            Direction::South => Vector([0, 1]),
            Direction::West => Vector([-1, 0]),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
struct Ship {
    position: Vector2D<i64>,
    waypoint: Vector2D<i64>,
}

impl Ship {
    fn new(waypoint: Vector2D<i64>) -> Self {
        Ship {
            position: Vector([0, 0]),
            waypoint,
        }
    }

    fn apply_directly(&mut self, action: Action) {
        match action {
            Action::Translate(direction, value) => {
                let direction: Vector2D<i64> = direction.into();
                let change = direction * value;
                self.position = self.position + change;
            }
            Action::Rotate(direction, value) => {
                let multiplier = match direction {
                    Rotation::Left => -1,
                    Rotation::Right => 1,
                };
                self.waypoint = rotate_vector(self.waypoint, (value / 90) * multiplier);
            }
            Action::Forward(value) => {
                self.position = self.position + self.waypoint * value;
            }
        }
    }

    fn apply_via_waypoint(&mut self, action: Action) {
        match action {
            Action::Translate(direction, value) => {
                let direction: Vector2D<i64> = direction.into();
                let change = direction * value;
                self.waypoint = self.waypoint + change;
            }
            Action::Rotate(direction, value) => {
                let multiplier = match direction {
                    Rotation::Left => -1,
                    Rotation::Right => 1,
                };
                self.waypoint = rotate_vector(self.waypoint, (value / 90) * multiplier);
            }
            Action::Forward(value) => {
                self.position = self.position + self.waypoint * value;
            }
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
    let mut state = Ship::new(Direction::East.into());
    for action in actions {
        state.apply_directly(action);
    }
    Ok(state.position.manhattan_length().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let actions = read_input(&input_path)?;
    let mut state = Ship::new(Vector([10, -1]));
    for action in actions {
        state.apply_via_waypoint(action.clone());
    }
    Ok(state.position.manhattan_length().to_string())
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
        assert_eq!(part1(data_path!("day12_example.txt")).unwrap(), "25");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day12_input.txt")).unwrap(), "1133");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(data_path!("day12_example.txt")).unwrap(), "286");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day12_input.txt")).unwrap(), "61053");
    }
}
