use std::collections::HashSet;

use super::prelude::*;
use crate::util;
use crate::vector::Vector;

type Point3D = Vector<[i64; 3]>;
type Point4D = Vector<[i64; 4]>;

/// Iterate points in the 3D volume bounded by `[min, max)`.
fn iter_volume_3d(min: Point3D, max: Point3D) -> impl Iterator<Item = Point3D> {
    (min[0]..max[0]).flat_map(move |x| {
        (min[1]..max[1]).flat_map(move |y| (min[2]..max[2]).map(move |z| Vector([x, y, z])))
    })
}

/// Iterate a 3x3x3 cube centered on `p`.
fn iter_adjacent_27(p: Point3D) -> impl Iterator<Item = Point3D> {
    iter_volume_3d(p - Vector([1, 1, 1]), p + Vector([2, 2, 2]))
}

/// Iterate a 3x3x3 cube centered on `p`, excluding `p` itself.
fn iter_adjacent_26(p: Point3D) -> impl Iterator<Item = Point3D> {
    iter_adjacent_27(p).filter(move |new| new != &p)
}

/// Iterate points in the 4D volume bounded by `[min, max)`.
fn iter_volume_4d(min: Point4D, max: Point4D) -> impl Iterator<Item = Point4D> {
    (min[0]..max[0]).flat_map(move |x| {
        (min[1]..max[1]).flat_map(move |y| {
            (min[2]..max[2]).flat_map(move |z| (min[3]..max[3]).map(move |w| Vector([x, y, z, w])))
        })
    })
}

/// Iterate a 3x3x3x3 cube centered on `p`.
fn iter_adjacent_81(p: Point4D) -> impl Iterator<Item = Point4D> {
    iter_volume_4d(p - Vector([1, 1, 1, 1]), p + Vector([2, 2, 2, 2]))
}

/// Iterate a 3x3x3x3 cube centered on `p`, excluding `p` itself.
fn iter_adjacent_80(p: Point4D) -> impl Iterator<Item = Point4D> {
    iter_adjacent_81(p).filter(move |new| new != &p)
}

struct SparseVolume3D {
    active: HashSet<Point3D>,
}

impl SparseVolume3D {
    fn new() -> Self {
        SparseVolume3D {
            active: HashSet::new(),
        }
    }

    fn count_active(&self) -> usize {
        self.active.len()
    }

    fn is_active(&self, p: Point3D) -> bool {
        self.active.contains(&p)
    }

    fn active_adjacent(&self, p: Point3D) -> usize {
        iter_adjacent_26(p)
            .map(|adj| self.active.contains(&adj))
            .filter(|active| *active)
            .count()
    }

    fn iter_relevant_points(&self) -> impl Iterator<Item = Point3D> + '_ {
        let mut seen: HashSet<Point3D> = HashSet::new();
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

impl std::fmt::Debug for SparseVolume3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min: Point3D = Vector([i64::max_value(), i64::max_value(), i64::max_value()]);
        let mut max: Point3D = Vector([i64::min_value(), i64::min_value(), i64::min_value()]);
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

struct SparseVolume4D {
    active: HashSet<Point4D>,
}

impl SparseVolume4D {
    fn new() -> Self {
        SparseVolume4D {
            active: HashSet::new(),
        }
    }

    fn count_active(&self) -> usize {
        self.active.len()
    }

    fn is_active(&self, p: Point4D) -> bool {
        self.active.contains(&p)
    }

    fn active_adjacent(&self, p: Point4D) -> usize {
        iter_adjacent_80(p)
            .map(|adj| self.active.contains(&adj))
            .filter(|active| *active)
            .count()
    }

    fn iter_relevant_points(&self) -> impl Iterator<Item = Point4D> + '_ {
        let mut seen: HashSet<Point4D> = HashSet::new();
        // Look at every active cell
        self.active
            .iter()
            // Expand to include every cell adjacent to an active cell, because by definition these
            // are the ones that are capable of changing across a time step
            .flat_map(|p| iter_adjacent_81(*p))
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

impl std::fmt::Debug for SparseVolume4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min: Point4D = Vector([
            i64::max_value(),
            i64::max_value(),
            i64::max_value(),
            i64::max_value(),
        ]);
        let mut max: Point4D = Vector([
            i64::min_value(),
            i64::min_value(),
            i64::min_value(),
            i64::min_value(),
        ]);
        for p in self.active.iter() {
            min = min.min(p);
            max = max.max(p);
        }
        writeln!(f, "min {:?} max {:?}", min, max)?;
        for w in min[3]..=max[3] {
            for z in min[2]..=max[2] {
                writeln!(f, "w = {}, z = {}", w, z)?;
                for y in min[1]..=max[1] {
                    for x in min[0]..=max[0] {
                        if self.is_active(Vector([x, y, z, w])) {
                            write!(f, "#")?;
                        } else {
                            write!(f, ".")?;
                        }
                    }
                    write!(f, "\n")?;
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn read_input_3d(input_path: &PathBuf) -> crate::Result<SparseVolume3D> {
    let mut volume = SparseVolume3D::new();
    for (y, line) in util::read_lines(input_path).enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                volume.active.insert(Vector([x as i64, y as i64, 0]));
            }
        }
    }
    Ok(volume)
}

fn read_input_4d(input_path: &PathBuf) -> crate::Result<SparseVolume4D> {
    let mut volume = SparseVolume4D::new();
    for (y, line) in util::read_lines(input_path).enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                volume.active.insert(Vector([x as i64, y as i64, 0, 0]));
            }
        }
    }
    Ok(volume)
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    let mut volume = read_input_3d(&input_path)?;
    for _ in 0..6 {
        volume.step();
    }
    Ok(volume.count_active().to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    let mut volume = read_input_4d(&input_path)?;
    for _ in 0..6 {
        volume.step();
    }
    Ok(volume.count_active().to_string())
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
        let adjacent: HashSet<Point3D> = iter_adjacent_26(Vector([0, 0, 0])).collect();
        let expected: HashSet<Point3D> = vec![
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
    fn test_part2_example1() {
        assert_eq!(part2(data_path!("day17_example1.txt")).unwrap(), "848");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day17_input.txt")).unwrap(), "1492");
    }
}
