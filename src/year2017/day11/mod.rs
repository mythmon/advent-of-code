use crate::cases::Puzzle;
use std::cmp;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

pub mod part1;
pub mod part2;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(part1::Part1), Box::new(part2::Part2)]
}

#[derive(Debug)]
pub struct HexVec {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl HexVec {
    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn size(&self) -> i32 {
        cmp::max(cmp::max(self.x.abs(), self.y.abs()), self.z.abs())
    }
}

impl<T> From<(T, T, T)> for HexVec
where
    T: Into<i32> + Copy,
{
    fn from(parts: (T, T, T)) -> Self {
        Self {
            x: parts.0.into(),
            y: parts.1.into(),
            z: parts.2.into(),
        }
    }
}

impl<T> Add<T> for HexVec
where
    T: Into<HexVec>,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        let other = other.into();
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign<T> for HexVec
where
    T: Into<HexVec>,
{
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug)]
pub enum HexDir {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for HexDir {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use self::HexDir::*;
        match input {
            "n" => Ok(North),
            "ne" => Ok(NorthEast),
            "nw" => Ok(NorthWest),
            "s" => Ok(South),
            "se" => Ok(SouthEast),
            "sw" => Ok(SouthWest),
            _ => Err(()),
        }
    }
}

impl Into<HexVec> for HexDir {
    fn into(self) -> HexVec {
        match self {
            HexDir::North => (0, 1, -1).into(),
            HexDir::NorthEast => (1, 0, -1).into(),
            HexDir::NorthWest => (-1, 1, 0).into(),
            HexDir::South => (0, -1, 1).into(),
            HexDir::SouthEast => (1, -1, 0).into(),
            HexDir::SouthWest => (-1, 0, 1).into(),
        }
    }
}
