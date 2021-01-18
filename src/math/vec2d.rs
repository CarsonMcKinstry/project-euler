use super::num::Num;
use std::convert::From;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct Vec2D<T: Num>(pub(crate) T, pub(crate) T);

impl<T: Num> From<(T, T)> for Vec2D<T> {
    fn from((a, b): (T, T)) -> Self {
        Self(a, b)
    }
}

impl Add for Vec2D<usize> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(a_1, a_2) = self;
        let Self(b_1, b_2) = rhs;

        Self(a_1 + b_1, a_2 + b_2)
    }
}

impl Mul<usize> for Vec2D<usize> {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        let Self(a_1, a_2) = self;

        Self(a_1 * rhs, a_2 * rhs)
    }
}

impl Mul<Vec2D<usize>> for Vec2D<usize> {
    type Output = usize;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self(a_1, a_2) = self;
        let Self(b_1, b_2) = rhs;

        a_1 * b_1 + a_2 * b_2
    }
}

impl Add for Vec2D<isize> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let Self(a_1, a_2) = self;
        let Self(b_1, b_2) = rhs;

        Self(a_1 + b_1, a_2 + b_2)
    }
}

impl Mul<isize> for Vec2D<isize> {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        let Self(a_1, a_2) = self;

        Self(a_1 * rhs, a_2 * rhs)
    }
}

impl Mul<Vec2D<isize>> for Vec2D<isize> {
    type Output = isize;

    fn mul(self, rhs: Self) -> Self::Output {
        let Self(a_1, a_2) = self;
        let Self(b_1, b_2) = rhs;

        a_1 * b_1 + a_2 * b_2
    }
}
