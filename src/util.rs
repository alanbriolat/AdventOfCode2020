use std::cmp;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::PathBuf;

use num;

#[macro_export]
macro_rules! data_path {
    ($filename:expr) => {{
        use std::env;
        use std::path::PathBuf;
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source_path = PathBuf::from(root_dir);
        source_path.push("data");
        source_path.push($filename);
        source_path
    }};
}

pub fn read_lines(path: &PathBuf) -> impl Iterator<Item = String> {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map(Result::unwrap)
}

/// Split `input` exactly once at `sep`, returning both sides of the split.
///
/// If `sep` is not found in `input`, returns `(input, "")`.
pub fn str_partition<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str) {
    match input.find(sep) {
        Some(pos) => (&input[..pos], &input[(pos + sep.len())..]),
        None => (input, ""),
    }
}

/// Like `str_partition` but from the right instead of the left.
///
/// If `sep` is not found in `input`, returns `("", input)`.
pub fn str_rpartition<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str) {
    match input.rfind(sep) {
        Some(pos) => (&input[..pos], &input[(pos + sep.len())..]),
        None => ("", input),
    }
}

pub trait Coord: num::Integer + num::CheckedSub + num::ToPrimitive + Copy + Debug {}
impl<T: num::Integer + num::CheckedSub + num::ToPrimitive + Copy + Debug> Coord for T {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector2D<C: Coord>(pub C, pub C);

impl<C: Coord> Vector2D<C> {
    pub fn min(&self, rhs: &Self) -> Self {
        Self(cmp::min(self.0, rhs.0), cmp::min(self.1, rhs.1))
    }

    pub fn max(&self, rhs: &Self) -> Self {
        Self(cmp::max(self.0, rhs.0), cmp::max(self.1, rhs.1))
    }
}

impl<C: Coord> ops::Add for Vector2D<C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<C: Coord> ops::Sub for Vector2D<C> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<C: Coord> num::CheckedSub for Vector2D<C> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Self(
            self.0.checked_sub(&rhs.0)?,
            self.1.checked_sub(&rhs.1)?,
        ))
    }
}

impl<C: Coord> ops::Mul<C> for Vector2D<C> {
    type Output = Self;

    fn mul(self, rhs: C) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl<C: Coord> ops::Rem for Vector2D<C> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0, self.1 % rhs.1)
    }
}

#[derive(Clone, Debug)]
pub struct Rect<C: Coord>(pub Vector2D<C>);

impl<C: Coord> Rect<C> {
    #[inline]
    pub fn size(&self) -> &Vector2D<C> {
        &self.0
    }

    pub fn area(&self) -> C {
        let Vector2D(w, h) = self.size().clone();
        w * h
    }

    pub fn contains(&self, point: Vector2D<C>) -> bool {
        let Vector2D(w, h) = self.size().clone();
        let Vector2D(x, y) = point;
        (C::zero()..w).contains(&x) && (C::zero()..h).contains(&y)
    }

    pub fn wrap_x(&self, point: Vector2D<C>) -> Vector2D<C> {
        Vector2D(point.0 % self.size().0, point.1)
    }

    pub fn row_major_index(&self, point: Vector2D<C>) -> Option<usize> {
        if self.contains(point) {
            let Vector2D(w, _h) = self.size().clone();
            let Vector2D(x, y) = point;
            Some((y * w + x).to_usize().unwrap())
        } else {
            None
        }
    }

    pub fn column_major_index(&self, point: Vector2D<C>) -> Option<usize> {
        if self.contains(point) {
            let Vector2D(_w, h) = self.size().clone();
            let Vector2D(x, y) = point;
            Some((x * h + y).to_usize().unwrap())
        } else {
            None
        }
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Vector2D<C>> {
        let Vector2D(w, h) = self.size().clone();
        num::range(C::zero(), h).into_iter().flat_map(move |y| {
            num::range(C::zero(), w)
                .into_iter()
                .map(move |x| Vector2D(x, y))
        })
    }
}

#[derive(Clone)]
pub struct Grid2D<T> {
    extent: Rect<i64>,
    data: Vec<T>,
}

pub const GRID2D_DIRECTIONS_4: [Vector2D<i64>; 4] = [
    Vector2D(-1, 0),
    Vector2D(0, -1),
    Vector2D(0, 1),
    Vector2D(1, 0),
];

pub const GRID2D_DIRECTIONS_8: [Vector2D<i64>; 8] = [
    Vector2D(-1, -1),
    Vector2D(-1, 0),
    Vector2D(-1, 1),
    Vector2D(0, -1),
    Vector2D(0, 1),
    Vector2D(1, -1),
    Vector2D(1, 0),
    Vector2D(1, 1),
];

impl<T> Grid2D<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> crate::Result<Self> {
        if rows.len() == 0 {
            return Err("at least one row required".into());
        }
        let size = Vector2D(rows[0].len() as i64, rows.len() as i64);
        let mut data = Vec::with_capacity((size.0 * size.1) as usize);
        for row in rows {
            if row.len() != size.0 as usize {
                return Err("all rows must have same width".into());
            }
            data.extend(row);
        }
        Ok(Self {
            extent: Rect(size),
            data,
        })
    }

    pub fn get(&self, point: Vector2D<i64>) -> Option<&T> {
        self.extent
            .row_major_index(point)
            .and_then(move |i| self.data.get(i))
    }

    pub fn get_mut(&mut self, point: Vector2D<i64>) -> Option<&mut T> {
        self.extent
            .row_major_index(point)
            .and_then(move |i| self.data.get_mut(i))
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Vector2D<i64>> {
        self.extent.iter_points()
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Vector2D<i64>, &T)> + '_ {
        self.iter_points().zip(self.data.iter())
    }

    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = (Vector2D<i64>, &mut T)> + '_ {
        self.iter_points().zip(self.data.iter_mut())
    }

    pub fn iter_adjacent_4(&self, point: Vector2D<i64>) -> impl Iterator<Item = Option<&T>> + '_ {
        GRID2D_DIRECTIONS_4
            .iter()
            .map(move |offset| self.get(point + *offset))
    }

    pub fn iter_adjacent_8(&self, point: Vector2D<i64>) -> impl Iterator<Item = Option<&T>> + '_ {
        GRID2D_DIRECTIONS_8
            .iter()
            .map(move |offset| self.get(point + *offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_partition() {
        assert_eq!(str_partition("a, b, c, d", ", "), ("a", "b, c, d"));
        assert_eq!(str_partition("a,b, c, d", ", "), ("a,b", "c, d"));
        assert_eq!(str_partition("a,b, c, d", " , "), ("a,b, c, d", ""));
    }

    #[test]
    fn test_str_rpartition() {
        assert_eq!(str_rpartition("a, b, c, d", ", "), ("a, b, c", "d"));
        assert_eq!(str_rpartition("a, b, c,d", ", "), ("a, b", "c,d"));
        assert_eq!(str_rpartition("a, b, c,d", " , "), ("", "a, b, c,d"));
    }
}
