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

pub fn str_partition<'a>(input: &'a str, sep: &str) -> (&'a str, &'a str) {
    match input.find(sep) {
        Some(pos) => (&input[..pos], &input[(pos + sep.len())..]),
        None => (input, ""),
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
        Some(Self(self.0.checked_sub(&rhs.0)?, self.1.checked_sub(&rhs.1)?))
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
}
