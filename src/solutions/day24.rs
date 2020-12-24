use std::collections::HashSet;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

use super::prelude::*;
use crate::util;
use crate::vector::{self, Vector};

type Vec2D = vector::Vector2D<i64>;

#[derive(Clone, Copy, Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    fn values() -> impl Iterator<Item = Direction> {
        static DIRECTIONS: [Direction; 6] = [
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NW,
            Direction::NE,
        ];
        DIRECTIONS.iter().copied()
    }
}

impl FromStr for Direction {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::E),
            "se" => Ok(Direction::SE),
            "sw" => Ok(Direction::SW),
            "w" => Ok(Direction::W),
            "nw" => Ok(Direction::NW),
            "ne" => Ok(Direction::NE),
            _ => Err(crate::Error::Other(format!(
                "unrecognised direction {:?}",
                s
            ))),
        }
    }
}

impl Into<Vec2D> for Direction {
    /// Represent the direction in 2D coordinate space.
    ///
    /// A hex grid can be represented as a square grid with additional connectivity. Here we map the
    /// hex grid directions to square grid offsets, like so:
    ///
    /// ```plaintext
    ///      -1   0    +1
    ///    +----+----+----+
    /// -1 | NW | NE |    |
    ///    +----+----+----+
    ///  0 | W  | X  | E  |
    ///    +----+----+----+
    /// +1 |    | SW | SE |
    ///    +----+----+----+
    /// ```
    fn into(self) -> Vec2D {
        match self {
            Direction::E => Vector([1, 0]),
            Direction::SE => Vector([1, 1]),
            Direction::SW => Vector([0, 1]),
            Direction::W => Vector([-1, 0]),
            Direction::NW => Vector([-1, -1]),
            Direction::NE => Vector([0, -1]),
        }
    }
}

#[derive(Clone, Debug)]
struct Directions(Vec<Direction>);

impl Directions {
    fn iter_path(&self, start: Vec2D) -> impl Iterator<Item = Vec2D> + '_ {
        std::iter::once(start).chain(self.0.iter().scan(start, |state, &d| {
            *state = *state + d;
            Some(*state)
        }))
    }

    fn apply(&self, start: Vec2D) -> Vec2D {
        self.iter_path(start).last().unwrap()
    }
}

impl std::ops::Add<Direction> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Direction) -> Self::Output {
        let offset: Vec2D = rhs.into();
        self + offset
    }
}

impl FromStr for Directions {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("e|se|sw|w|nw|ne").unwrap();
        }
        RE.find_iter(s)
            .map(|m| m.as_str().parse())
            .collect::<Result<Vec<_>, _>>()
            .map(Directions)
    }
}

struct Floor {
    black_tiles: HashSet<Vec2D>,
}

impl Floor {
    fn new() -> Self {
        Floor {
            black_tiles: HashSet::new(),
        }
    }

    fn flip_tile(&mut self, tile: Vec2D) {
        if !self.black_tiles.remove(&tile) {
            self.black_tiles.insert(tile);
        }
    }

    fn num_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Vec<Directions>> {
    util::read_lines(input_path)
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let input = read_input(&input_path)?;
    let mut floor = Floor::new();
    for directions in input {
        floor.flip_tile(directions.apply(Vector([0, 0])));
    }
    Ok(floor.num_black_tiles().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day24part1", || part1(data_path!("day24_input.txt")));
    runner.add("day24part2", || part2(data_path!("day24_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directions_apply() {
        assert_eq!(
            "nwwswee"
                .parse::<Directions>()
                .unwrap()
                .apply(Vector([0, 0])),
            Vector([0, 0])
        );
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day24_example1.txt")).unwrap(), "10");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day24_input.txt")).unwrap(), "424");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day24_input.txt")).unwrap(), "");
    }
}
