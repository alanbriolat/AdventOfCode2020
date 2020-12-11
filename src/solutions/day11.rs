use std::ops::{Deref, DerefMut};

use super::prelude::*;
use crate::util::{self, Vector2D};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

impl From<u8> for Tile {
    fn from(b: u8) -> Self {
        match b {
            b'.' => Tile::Floor,
            b'L' => Tile::Empty,
            b'#' => Tile::Occupied,
            _ => panic!("unrecognised tile: {}", b),
        }
    }
}

#[derive(Clone)]
struct Map(util::Grid2D<Tile>);

impl Deref for Map {
    type Target = util::Grid2D<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    fn count_adjacent_occupied(&self, point: Vector2D<i64>) -> usize {
        self.iter_adjacent_8(point)
            .filter(|t| matches!(t, Some(Tile::Occupied)))
            .count()
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<Map> {
    let data: Vec<Vec<Tile>> = util::read_lines(input_path)
        .map(|line| line.bytes().map(Tile::from).collect::<Vec<_>>())
        .collect();
    Ok(Map(util::Grid2D::from_rows(data)?))
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let mut map = read_input(&input_path)?;
    loop {
        let mut new_map = map.clone();
        let mut changes = 0_usize;
        for (p, t) in new_map.iter_cells_mut() {
            match t {
                Tile::Empty => {
                    if map.count_adjacent_occupied(p) == 0 {
                        *t = Tile::Occupied;
                        changes += 1;
                    }
                }
                Tile::Occupied => {
                    if map.count_adjacent_occupied(p) >= 4 {
                        *t = Tile::Empty;
                        changes += 1;
                    }
                }
                _ => {}
            }
        }
        if changes == 0 {
            break;
        }
        map = new_map;
    }
    let occupied_count = map
        .iter_cells()
        .filter(|(_p, t)| matches!(t, Tile::Occupied))
        .count();
    Ok(occupied_count.to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day11part1", || part1(data_path!("day11_input.txt")));
    runner.add("day11part2", || part2(data_path!("day11_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(data_path!("day11_example.txt")).unwrap(), "37");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day11_input.txt")).unwrap(), "2354");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day11_input.txt")).unwrap(), "");
    }
}
