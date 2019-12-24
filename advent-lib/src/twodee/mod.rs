use num_traits::{sign::Signed, Num};
use std::{cmp, fmt, iter, ops};

mod grid;

pub use self::grid::Grid;
pub use self::grid::HashGrid;

pub trait PointAxe: fmt::Debug + Clone + Copy + Eq + Num {}
impl<T> PointAxe for T where T: fmt::Debug + Clone + Copy + Eq + Num {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T: PointAxe> {
    pub x: T,
    pub y: T,
}

impl<T: PointAxe> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn manhattan_magnitude(&self) -> T {
        self.x + self.y
    }

    pub fn direction_to(&self, rhs: Point<T>) -> Option<Dir> {
        let zero = T::zero();
        let one = T::one();
        let negative_one = zero - one;
        match rhs - *self {
            Self { x, y } if x == zero && y == negative_one => Some(Dir::Up),
            Self { x, y } if x == zero && y == one => Some(Dir::Down),
            Self { x, y } if x == negative_one && y == zero => Some(Dir::Left),
            Self { x, y } if x == one && y == zero => Some(Dir::Right),
            _ => None,
        }
    }

    pub fn neighbors4(&self) -> [Self; 4] {
        [
            Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Self {
                x: self.x,
                y: self.y - T::one(),
            },
            Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Self {
                x: self.x - T::one(),
                y: self.y,
            },
        ]
    }

    #[must_use]
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: PointAxe + Signed> Point<T> {
    pub fn manhattan_distance(&self, rhs: Point<T>) -> T {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
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
    T: PointAxe + ops::Sub<T, Output = T>,
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
    T: PointAxe + ops::Add<R, Output = O>,
    R: PointAxe,
    O: PointAxe,
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

impl<T: PointAxe> ops::Add<Dir> for Point<T> {
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
    T: PointAxe,
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
    T: PointAxe + ops::Mul<R, Output = O>,
    R: PointAxe,
    O: PointAxe,
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
    T: PointAxe + ops::Div<R, Output = O>,
    R: PointAxe,
    O: PointAxe,
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

impl<T> fmt::Display for Point<T>
where
    T: PointAxe + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)?;
        Ok(())
    }
}

pub struct Bounds<I> {
    top: I,
    left: I,
    right: I,
    bottom: I,
}

impl<I> Bounds<I>
where
    I: PointAxe,
{
    fn top_left(&self) -> Point<I> {
        Point::new(self.left, self.top)
    }

    fn bottom_right(&self) -> Point<I> {
        Point::new(self.right, self.bottom)
    }
}

impl<I> iter::FromIterator<Point<I>> for Bounds<I>
where
    I: PointAxe + cmp::Ord,
{
    fn from_iter<T: IntoIterator<Item = Point<I>>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let (mut top, mut right, mut bottom, mut left) = match iter.next() {
            Some(first) => (first.y, first.x, first.y, first.x),
            None => (I::zero(), I::zero(), I::zero(), I::zero()),
        };

        for point in iter {
            left = cmp::min(left, point.x);
            right = cmp::max(right, point.x);
            top = cmp::min(top, point.y);
            bottom = cmp::max(bottom, point.y);
        }

        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}
