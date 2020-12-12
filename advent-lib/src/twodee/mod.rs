use num_traits::{sign::Signed, Num};
use std::{cmp, fmt, iter, ops, ops::MulAssign};

mod grid;

pub use self::grid::Grid;
pub use self::grid::HashGrid;
pub use self::grid::VecGrid;

pub trait PointAxe: fmt::Debug + Clone + Copy + Eq + Num + PartialOrd {}
impl<T> PointAxe for T where T: fmt::Debug + Clone + Copy + Eq + Num + PartialOrd {}

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

    pub fn direction4_to(&self, rhs: Point<T>) -> Option<Dir4> {
        let zero = T::zero();
        let one = T::one();
        let negative_one = zero - one;
        match rhs - *self {
            Self { x, y } if x == zero && y == negative_one => Some(Dir4::Up),
            Self { x, y } if x == zero && y == one => Some(Dir4::Down),
            Self { x, y } if x == negative_one && y == zero => Some(Dir4::Left),
            Self { x, y } if x == one && y == zero => Some(Dir4::Right),
            _ => None,
        }
    }

    pub fn direction8_to(&self, rhs: Point<T>) -> Option<Dir8> {
        let zero = T::zero();
        let one = T::one();
        let negative_one = zero - one;
        match rhs - *self {
            Self { x, y } if x == zero && y == negative_one => Some(Dir8::Up),
            Self { x, y } if x == one && y == negative_one => Some(Dir8::UpRight),
            Self { x, y } if x == one && y == zero => Some(Dir8::Right),
            Self { x, y } if x == one && y == one => Some(Dir8::DownRight),
            Self { x, y } if x == zero && y == one => Some(Dir8::Down),
            Self { x, y } if x == negative_one && y == one => Some(Dir8::DownLeft),
            Self { x, y } if x == negative_one && y == zero => Some(Dir8::Left),
            Self { x, y } if x == negative_one && y == negative_one => Some(Dir8::UpLeft),
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

    pub fn neighbors8(&self) -> [Self; 8] {
        [
            Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Self {
                x: self.x + T::one(),
                y: self.y + T::one(),
            },
            Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Self {
                x: self.x + T::one(),
                y: self.y - T::one(),
            },
            Self {
                x: self.x,
                y: self.y - T::one(),
            },
            Self {
                x: self.x - T::one(),
                y: self.y - T::one(),
            },
            Self {
                x: self.x - T::one(),
                y: self.y,
            },
            Self {
                x: self.x - T::one(),
                y: self.y + T::one(),
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
pub enum Dir4 {
    Up,
    Down,
    Left,
    Right,
}

impl Dir4 {
    pub fn spin_iter() -> SpinIterator<Self> {
        SpinIterator(Self::Up)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir8 {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Dir8 {
    pub fn spin_iter() -> SpinIterator<Self> {
        SpinIterator(Self::Up)
    }
}

pub struct SpinIterator<D>(D);

impl<D> Iterator for SpinIterator<D>
where
    D: MulAssign<Turn> + Clone,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.clone();
        self.0 *= Turn::Cw;
        Some(next)
    }
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

impl<T: PointAxe> ops::Add<Dir4> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Dir4) -> Self::Output {
        #![allow(clippy::suspicious_arithmetic_impl)]
        use Dir4::{Down, Left, Right, Up};
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

impl<T: PointAxe> ops::Add<Dir8> for Point<T> {
    type Output = Self;

    fn add(self, rhs: Dir8) -> Self::Output {
        match rhs {
            Dir8::Up => Self {
                x: self.x,
                y: self.y - T::one(),
            },
            Dir8::UpRight => Self {
                x: self.x + T::one(),
                y: self.y - T::one(),
            },
            Dir8::Right => Self {
                x: self.x + T::one(),
                y: self.y,
            },
            Dir8::DownRight => Self {
                x: self.x + T::one(),
                y: self.y + T::one(),
            },
            Dir8::Down => Self {
                x: self.x,
                y: self.y + T::one(),
            },
            Dir8::DownLeft => Self {
                x: self.x - T::one(),
                y: self.y + T::one(),
            },
            Dir8::Left => Self {
                x: self.x - T::one(),
                y: self.y,
            },
            Dir8::UpLeft => Self {
                x: self.x - T::one(),
                y: self.y - T::one(),
            },
        }
    }
}

impl<T> ops::AddAssign<Dir4> for Point<T>
where
    T: PointAxe,
    Point<T>: ops::Add<Dir4, Output = Point<T>>,
{
    fn add_assign(&mut self, rhs: Dir4) {
        *self = *self + rhs;
    }
}

impl<T> ops::AddAssign<Dir8> for Point<T>
where
    T: PointAxe,
    Point<T>: ops::Add<Dir8, Output = Point<T>>,
{
    fn add_assign(&mut self, rhs: Dir8) {
        *self = *self + rhs;
    }
}

impl ops::Mul<Turn> for Dir4 {
    type Output = Self;

    fn mul(self, rhs: Turn) -> Self::Output {
        use Dir4::{Down, Left, Right, Up};
        use Turn::{Ccw, Cw, Flip};
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

impl ops::MulAssign<Turn> for Dir4 {
    fn mul_assign(&mut self, rhs: Turn) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Turn> for Dir8 {
    type Output = Self;

    fn mul(self, rhs: Turn) -> Self::Output {
        #[allow(clippy::match_same_arms)]
        match (self, rhs) {
            (Dir8::Up, Turn::Cw) => Dir8::UpRight,
            (Dir8::Up, Turn::Ccw) => Dir8::UpLeft,
            (Dir8::Up, Turn::Flip) => Dir8::Down,
            (Dir8::UpRight, Turn::Cw) => Dir8::Right,
            (Dir8::UpRight, Turn::Ccw) => Dir8::Up,
            (Dir8::UpRight, Turn::Flip) => Dir8::DownLeft,
            (Dir8::Right, Turn::Cw) => Dir8::DownRight,
            (Dir8::Right, Turn::Ccw) => Dir8::UpRight,
            (Dir8::Right, Turn::Flip) => Dir8::Left,
            (Dir8::DownRight, Turn::Cw) => Dir8::Down,
            (Dir8::DownRight, Turn::Ccw) => Dir8::Right,
            (Dir8::DownRight, Turn::Flip) => Dir8::UpLeft,
            (Dir8::Down, Turn::Cw) => Dir8::DownLeft,
            (Dir8::Down, Turn::Ccw) => Dir8::DownRight,
            (Dir8::Down, Turn::Flip) => Dir8::Up,
            (Dir8::DownLeft, Turn::Cw) => Dir8::Left,
            (Dir8::DownLeft, Turn::Ccw) => Dir8::Down,
            (Dir8::DownLeft, Turn::Flip) => Dir8::UpRight,
            (Dir8::Left, Turn::Cw) => Dir8::UpLeft,
            (Dir8::Left, Turn::Ccw) => Dir8::DownLeft,
            (Dir8::Left, Turn::Flip) => Dir8::Right,
            (Dir8::UpLeft, Turn::Cw) => Dir8::Up,
            (Dir8::UpLeft, Turn::Ccw) => Dir8::Left,
            (Dir8::UpLeft, Turn::Flip) => Dir8::DownRight,
        }
    }
}

impl ops::MulAssign<Turn> for Dir8 {
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn new(top: I, left: I, right: I, bottom: I) -> Self {
        assert!(top <= bottom);
        assert!(left <= right);
        Self {
            top,
            left,
            right,
            bottom,
        }
    }

    pub fn top_left(&self) -> Point<I> {
        Point::new(self.left, self.top)
    }

    pub fn bottom_right(&self) -> Point<I> {
        Point::new(self.right, self.bottom)
    }

    pub fn width(&self) -> I {
        self.right - self.left
    }

    pub fn height(&self) -> I {
        self.bottom - self.top
    }

    pub fn contains(&self, p: Point<I>) -> bool {
        (self.left..self.right).contains(&p.x) && (self.top..self.bottom).contains(&p.y)
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
