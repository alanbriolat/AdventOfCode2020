use std::collections::HashSet;

use super::prelude::*;
use crate::util;
use crate::vector::Vector;

type Point = Vector<[i64; 3]>;

/// Iterate points in the 3D volume bounded by `[min, max)`.
fn iter_volume(min: Point, max: Point) -> impl Iterator<Item = Point> {
    (min[0]..max[0]).flat_map(move |x| {
        (min[1]..max[1]).flat_map(move |y| (min[2]..max[2]).map(move |z| Vector([x, y, z])))
    })
}

/// Iterate a 3x3x3 cube centered on `p`.
fn iter_adjacent_27(p: Point) -> impl Iterator<Item = Point> {
    iter_volume(p - Vector([1, 1, 1]), p + Vector([2, 2, 2]))
}

/// Iterate a 3x3x3 cube centered on `p`, excluding `p` itself.
fn iter_adjacent_26(p: Point) -> impl Iterator<Item = Point> {
    iter_adjacent_27(p).filter(move |new| new != &p)
}

struct SparseVolume {
    active: HashSet<Point>,
}

impl SparseVolume {
    fn new() -> Self {
        SparseVolume { active: HashSet::new() }
    }

    fn count_active(&self) -> usize {
        self.active.len()
    }

    fn is_active(&self, p: Point) -> bool {
        self.active.contains(&p)
    }

    fn active_adjacent(&self, p: Point) -> usize {
        iter_adjacent_26(p)
            .map(|adj| self.active.contains(&adj))
            .filter(|active| *active)
            .count()
    }

    fn iter_relevant_points(&self) -> impl Iterator<Item = Point> + '_ {
        let mut seen: HashSet<Point> = HashSet::new();
        // Look at every active cell
        self.active
            .iter()
            // Expand to include every cell adjacent to an active cell, because by definition these
            // are the ones that are capable of changing across a time step
            .flat_map(|p| iter_adjacent_27(*p))
            // De-duplicate points, because there will be a lot of overlap
            .filter(move |p| {
                if seen.contains(p) {
                    false
                } else {
                    seen.insert(*p);
                    true
                }
            })
    }

    fn step(&mut self) {
        let mut new_active = HashSet::new();
        for p in self.iter_relevant_points() {
            let state = (self.is_active(p), self.active_adjacent(p));
            let active = match state {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
            if active {
                new_active.insert(p);
            }
        }
        self.active = new_active;
    }
}

impl std::fmt::Debug for SparseVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min: Point = Vector([i64::max_value(), i64::max_value(), i64::max_value()]);
        let mut max: Point = Vector([i64::min_value(), i64::min_value(), i64::min_value()]);
        for p in self.active.iter() {
            min = min.min(p);
            max = max.max(p);
        }
        writeln!(f, "min {:?} max {:?}", min, max)?;
        for z in min[2]..=max[2] {
            writeln!(f, "z = {}", z)?;
            for y in min[1]..=max[1] {
                for x in min[0]..=max[0] {
                    if self.is_active(Vector([x, y, z])) {
                        write!(f, "#")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
                write!(f, "\n")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn read_input(input_path: &PathBuf) -> crate::Result<SparseVolume> {
    let mut volume = SparseVolume::new();
    for (y, line) in util::read_lines(input_path).enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                volume.active.insert(Vector([x as i64, y as i64, 0]));
            }
        }
    }
    Ok(volume)
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let mut volume = read_input(&input_path)?;
    for _ in 0..6 {
        volume.step();
    }
    Ok(volume.count_active().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day17part1", || part1(data_path!("day17_input.txt")));
    runner.add("day17part2", || part2(data_path!("day17_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_adjacent_26() {
        let adjacent: HashSet<Point> = iter_adjacent_26(Vector([0, 0, 0])).collect();
        let expected: HashSet<Point> = vec![
            Vector([-1, -1, -1]),
            Vector([-1, -1, 0]),
            Vector([-1, -1, 1]),
            Vector([-1, 0, -1]),
            Vector([-1, 0, 0]),
            Vector([-1, 0, 1]),
            Vector([-1, 1, -1]),
            Vector([-1, 1, 0]),
            Vector([-1, 1, 1]),
            Vector([0, -1, -1]),
            Vector([0, -1, 0]),
            Vector([0, -1, 1]),
            Vector([0, 0, -1]),
            // Vector([0, 0, 0]),
            Vector([0, 0, 1]),
            Vector([0, 1, -1]),
            Vector([0, 1, 0]),
            Vector([0, 1, 1]),
            Vector([1, -1, -1]),
            Vector([1, -1, 0]),
            Vector([1, -1, 1]),
            Vector([1, 0, -1]),
            Vector([1, 0, 0]),
            Vector([1, 0, 1]),
            Vector([1, 1, -1]),
            Vector([1, 1, 0]),
            Vector([1, 1, 1]),
        ]
        .into_iter()
        .collect();
        assert_eq!(adjacent, expected);
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(part1(data_path!("day17_example1.txt")).unwrap(), "112");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(part1(data_path!("day17_input.txt")).unwrap(), "209");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day17_input.txt")).unwrap(), "");
    }
}
