use std::{
    iter::Iterator,
    ops::{Index, IndexMut},
    str,
};

pub trait StringAdventExt {
    fn trimmed_lines(&self) -> TrimmedLines;
}

impl StringAdventExt for &str {
    fn trimmed_lines(&self) -> TrimmedLines {
        TrimmedLines(self.lines())
    }
}

impl StringAdventExt for String {
    fn trimmed_lines(&self) -> TrimmedLines {
        TrimmedLines(self.lines())
    }
}

#[derive(Debug, Clone)]
pub struct TrimmedLines<'a>(std::str::Lines<'a>);

impl<'a> Iterator for TrimmedLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        while let Some(l) = self.0.next() {
            let trimmed = l.trim();
            if trimmed != "" {
                return Some(trimmed);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    storage: Vec<T>,
    stride: usize,
}

impl<T> Grid<T> {
    #[must_use]
    pub const fn width(&self) -> usize {
        self.stride
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.storage.len() / self.stride
    }

    #[must_use]
    pub fn iter_coordinates(&self) -> GridCoordinateIterator {
        GridCoordinateIterator {
            state: 0,
            width: self.width(),
            height: self.height(),
        }
    }

    #[must_use]
    pub const fn iter_values(&self) -> GridValIterator<'_, T> {
        GridValIterator {
            state: 0,
            grid: self,
        }
    }
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut storage = Vec::with_capacity(size);
        storage.resize_with(size, Default::default);
        Self {
            storage,
            stride: width,
        }
    }
}

pub struct GridCoordinateIterator {
    state: usize,
    width: usize,
    height: usize,
}

impl Iterator for GridCoordinateIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.state % self.width;
        let y = self.state / self.height;
        if y >= self.height {
            None
        } else {
            self.state += 1;
            Some((x, y).into())
        }
    }
}

pub struct GridValIterator<'a, T> {
    state: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> Iterator for GridValIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state >= self.grid.storage.len() {
            None
        } else {
            let rv = &self.grid.storage[self.state];
            self.state += 1;
            Some(rv)
        }
    }
}

impl<T, I> Index<I> for Grid<T>
where
    I: Into<Point>,
{
    type Output = T;

    fn index(&self, p: I) -> &T {
        let p = p.into();
        self.storage.index(p.x + p.y * self.stride)
    }
}

impl<T, I> IndexMut<I> for Grid<T>
where
    I: Into<Point>,
{
    fn index_mut(&mut self, p: I) -> &mut T {
        let p = p.into();
        self.storage.index_mut(p.x + p.y * self.stride)
    }
}

impl<T> std::fmt::Display for Grid<Option<T>>
where
    T: std::fmt::Display,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn symbol_for<T: std::fmt::Display>(v: &Option<T>) -> String {
            match v {
                Some(v) => format!("{}", v),
                None => ".".to_owned(),
            }
        }

        let max_width = self
            .iter_values()
            .map(|v| symbol_for(v).len())
            .max()
            .unwrap();

        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(
                    fmt,
                    "{:^width$}",
                    symbol_for(&self[(x, y)]),
                    width = max_width
                )?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn manhattan_distance<'a, T: Into<&'a Self>>(&self, other: T) -> usize {
        let other: &Self = other.into();
        self.x.difference(&other.x) + self.y.difference(&other.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "({}, {})", self.x, self.y)
    }
}

impl std::str::FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(',').map(str::trim).collect();
        if parts.len() == 2 {
            Ok(Self {
                x: parts[0].parse()?,
                y: parts[1].parse()?,
            })
        } else {
            Err(From::from("Points must be 2d"))
        }
    }
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

trait Difference {
    type Out = usize;
    fn difference(&self, other: &Self) -> Self::Out;
}

impl Difference for usize {
    type Out = Self;
    fn difference(&self, other: &Self) -> Self::Out {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}
