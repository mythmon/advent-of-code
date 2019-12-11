use num_traits::{One, Zero};
use std::{fmt::Debug, ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T>
where
    T: Debug + Clone + Copy + Eq,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Debug + Clone + Copy + Eq,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Point<T>
where
    T: Debug + Clone + Copy + Eq + Zero,
{
    #[must_use]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    Cw,
    Ccw,
    Flip,
}

impl<T> ops::Sub<Point<T>> for Point<T>
where
    T: Debug + Clone + Copy + Eq + ops::Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T, R, O> ops::Add<Point<R>> for Point<T>
where
    T: ops::Add<R, Output = O> + Debug + Clone + Copy + Eq,
    R: Debug + Clone + Copy + Eq,
    O: Debug + Clone + Copy + Eq,
{
    #![allow(clippy::use_self)]
    type Output = Point<O>;

    fn add(self, rhs: Point<R>) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Add<Dir> for Point<T>
where
    T: Debug + Clone + Copy + Eq + ops::Add<T, Output = T> + ops::Sub<T, Output = T> + One,
{
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        #![allow(clippy::suspicious_arithmetic_impl)]
        use Dir::*;
        match rhs {
            Up => Self {
                x: self.x,
                y: self.y - T::one(),
            },
            Down => Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Left => Self {
                x: self.x - T::one(),
                y: self.y,
            },
            Right => Self {
                x: self.x + T::one(),
                y: self.y,
            },
        }
    }
}

impl<T> ops::AddAssign<Dir> for Point<T>
where
    T: Copy + Clone + Debug + Eq,
    Point<T>: ops::Add<Dir, Output = Point<T>>,
{
    fn add_assign(&mut self, rhs: Dir) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Turn> for Dir {
    type Output = Self;

    fn mul(self, rhs: Turn) -> Self::Output {
        use Dir::*;
        use Turn::*;
        #[allow(clippy::match_same_arms)]
        match (self, rhs) {
            (Up, Cw) => Right,
            (Up, Ccw) => Left,
            (Up, Flip) => Down,
            (Down, Cw) => Left,
            (Down, Ccw) => Right,
            (Down, Flip) => Up,
            (Right, Cw) => Down,
            (Right, Ccw) => Up,
            (Right, Flip) => Left,
            (Left, Cw) => Up,
            (Left, Ccw) => Down,
            (Left, Flip) => Right,
        }
    }
}

impl ops::MulAssign<Turn> for Dir {
    fn mul_assign(&mut self, rhs: Turn) {
        *self = *self * rhs;
    }
}

impl<T, R, O> ops::Mul<R> for Point<T>
where
    T: Copy + Clone + Debug + Eq + ops::Mul<R, Output = O>,
    R: Copy,
    O: Copy + Clone + Debug + Eq,
{
    #![allow(clippy::use_self)]
    type Output = Point<O>;

    fn mul(self, rhs: R) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T, R, O> ops::Div<R> for Point<T>
where
    T: Copy + Clone + Debug + Eq + ops::Div<R, Output = O>,
    R: Copy,
    O: Copy + Clone + Debug + Eq,
{
    #![allow(clippy::use_self)]
    type Output = Point<O>;

    fn div(self, rhs: R) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

macro_rules! point_from {
    ($from: ty, $into: ty) => {
        impl From<Point<$from>> for Point<$into> {
            fn from(other: Point<$from>) -> Self {
                Self {
                    x: other.x as $into,
                    y: other.y as $into,
                }
            }
        }
    };
}

point_from!(usize, i32);
point_from!(i32, usize);
