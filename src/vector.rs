use std::cmp;
use std::fmt;
use std::ops;

use num::{Signed, Zero};

pub trait Coord: num::Integer + num::CheckedSub + num::ToPrimitive + Copy + fmt::Debug {}
impl<T: num::Integer + num::CheckedSub + num::ToPrimitive + Copy + fmt::Debug> Coord for T {}

pub trait VectorInner {
    type Item;

    const SIZE: usize;

    fn as_slice(&self) -> &[Self::Item];

    fn as_mut_slice(&mut self) -> &mut [Self::Item];
}

impl<C> VectorInner for [C; 2] {
    type Item = C;

    const SIZE: usize = 2;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_ref()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }
}

impl<C> VectorInner for [C; 3] {
    type Item = C;

    const SIZE: usize = 3;

    fn as_slice(&self) -> &[Self::Item] {
        self.as_ref()
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        self.as_mut()
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Vector<T>(pub T);

pub type Vector2D<C> = Vector<[C; 2]>;
pub type Vector3D<C> = Vector<[C; 3]>;

impl<T> ops::Deref for Vector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: VectorInner> ops::Index<usize> for Vector<T> {
    type Output = T::Item;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T: VectorInner> ops::IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({:?})", self.0)
    }
}

impl<T> ops::Add for Vector<T>
where
    T: VectorInner,
    <T as VectorInner>::Item: Coord,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..T::SIZE {
            self[i] = self[i] + rhs[i];
        }
        self
    }
}

impl<T> ops::Sub for Vector<T>
where
    T: VectorInner,
    <T as VectorInner>::Item: Coord,
{
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..T::SIZE {
            self[i] = self[i] - rhs[i];
        }
        self
    }
}

impl<T> num::CheckedSub for Vector<T>
where
    T: VectorInner + Clone,
    <T as VectorInner>::Item: Coord + num::CheckedSub,
{
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let mut new = self.clone();
        for i in 0..T::SIZE {
            new[i] = self[i].checked_sub(&rhs[i])?;
        }
        Some(new)
    }
}

impl<T> ops::Mul<T::Item> for Vector<T>
where
    T: VectorInner,
    <T as VectorInner>::Item: Coord,
{
    type Output = Self;

    fn mul(mut self, rhs: T::Item) -> Self::Output {
        for i in 0..T::SIZE {
            self[i] = self[i] * rhs;
        }
        self
    }
}

impl<T> ops::Rem for Vector<T>
where
    T: VectorInner,
    <T as VectorInner>::Item: Coord,
{
    type Output = Self;

    fn rem(mut self, rhs: Self) -> Self::Output {
        for i in 0..T::SIZE {
            self[i] = self[i] % rhs[i];
        }
        self
    }
}

impl<T> Vector<T>
where
    T: VectorInner + Clone,
    <T as VectorInner>::Item: Coord,
{
    pub fn min(&self, rhs: &Self) -> Self {
        let mut new = self.clone();
        for i in 0..T::SIZE {
            new[i] = cmp::min(new[i], rhs[i]);
        }
        new
    }

    pub fn max(&self, rhs: &Self) -> Self {
        let mut new = self.clone();
        for i in 0..T::SIZE {
            new[i] = cmp::max(new[i], rhs[i]);
        }
        new
    }
}

impl<T> Vector<T>
where
    T: VectorInner,
    <T as VectorInner>::Item: Coord + num::Signed + num::Zero,
{
    pub fn manhattan_length(&self) -> T::Item {
        let mut sum = T::Item::zero();
        for i in 0..T::SIZE {
            sum = sum + self[i].abs();
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use num::CheckedSub;

    use super::*;

    type Vec2D = Vector<[i64; 2]>;
    type UVec2D = Vector<[u64; 2]>;

    #[test]
    fn test_vector_elementwise_arithmetic() {
        let v1: Vec2D = Vector([10, 20]);
        let v2: Vec2D = Vector([2, 3]);

        assert_eq!(v1 + v2, Vector([12, 23]));
        assert_eq!(v1 - v2, Vector([8, 17]));
        assert_eq!(v1 % v2, Vector([0, 2]));
    }

    #[test]
    fn test_vector_min_max() {
        let v1: Vec2D = Vector([20, -10]);
        let v2: Vec2D = Vector([-2, 5]);

        assert_eq!(v1.min(&v2), Vector([-2, -10]));
        assert_eq!(v1.max(&v2), Vector([20, 5]));
        assert_eq!(v1.min(&v2), v2.min(&v1));
        assert_eq!(v1.max(&v2), v2.max(&v1));
    }

    #[test]
    fn test_vector_scaling() {
        assert_eq!(Vector([3, -4]) * 5, Vector([15, -20]));
    }

    #[test]
    fn test_vector_checked_sub() {
        let v1: UVec2D = Vector([10, 5]);
        let v2: UVec2D = Vector([22, 15]);

        assert_eq!(v1.checked_sub(&v2), None);
        assert_eq!(v2.checked_sub(&v1), Some(Vector([12, 10])));
    }
}
