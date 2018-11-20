use std::collections::HashMap;
use std::ops::{Add, AddAssign};

const INPUT: u32 = 289326;

fn main() {
    println!("{}", puzzle(INPUT));
}

fn puzzle(input: u32) -> u32 {
    GridStressValues::new()
        .skip_while(|v| v <= &input)
        .next()
        .unwrap()
}

struct GridStressValues {
    vals: HashMap<GridCoord, u32>,
    coords: SpiralCoords,
}

impl GridStressValues {
    fn new() -> Self {
        let mut vals = HashMap::new();
        vals.insert(GridCoord::new(0, 0), 1);
        Self {
            vals: vals,
            coords: SpiralCoords::new(),
        }
    }
}

impl Iterator for GridStressValues {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.coords.next().unwrap();
        if self.vals.contains_key(&p) {
            Some(*self.vals.get(&p).unwrap())
        } else {
            let s = p.neighbors()
                .iter()
                .map(|n| self.vals.get(&n).unwrap_or(&0))
                .sum();
            self.vals.insert(p, s);
            Some(s)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridCoord {
    x: i32,
    y: i32,
}

impl GridCoord {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    fn neighbors(self) -> Vec<Self> {
        let mut rv = Vec::with_capacity(9);
        for dx in -1..2 {
            for dy in -1..2 {
                rv.push(Self {
                    x: self.x + dx,
                    y: self.y + dy,
                })
            }
        }
        rv
    }
}

impl Add<Direction> for GridCoord {
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        Self::Output {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl AddAssign<Direction> for GridCoord {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Debug)]
struct SpiralCoords {
    dir: Direction,
    pos: GridCoord,
    stride: u32,
    idx: u32,
    parity: bool,
}

impl SpiralCoords {
    fn new() -> Self {
        Self {
            dir: Direction::new(1, 0),
            pos: GridCoord::new(0, 0),
            stride: 1,
            idx: 0,
            parity: false,
        }
    }
}

impl Iterator for SpiralCoords {
    type Item = GridCoord;

    fn next(&mut self) -> Option<Self::Item> {
        let rv = self.pos;

        self.pos += self.dir;
        self.idx += 1;

        if self.idx >= self.stride {
            self.dir = self.dir.rotate90();
            self.idx = 0;
            if self.parity {
                self.stride += 1;
                self.parity = false;
            } else {
                self.parity = true;
            }
        }

        Some(rv)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    fn new(x: i32, y: i32) -> Self {
        if x < -1 || x > 1 || y < -1 || y > 1 || (x == 0 && y == 0) {
            panic!(format!("Invalid direction ({}, {})", x, y));
        }
        Self { x: x, y: y }
    }

    fn rotate90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

#[test]
fn test_correct_answer() {
    assert_eq!(puzzle(INPUT), 295229);
}

#[test]
fn test_grid_stress_values() {
    let expected = vec![
        1,
        1,
        2,
        4,
        5,
        10,
        11,
        23,
        25,
        26,
        54,
        57,
        59,
        122,
        133,
        142,
        147,
        304,
        330,
        351,
        362,
        747,
        806,
    ];
    let actual: Vec<u32> = GridStressValues::new().take(expected.len()).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_spiral_coords() {
    let expected = vec![
        (0, 0),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (2, -1),
        (2, 0),
        (2, 1),
        (2, 2),
        (1, 2),
        (0, 2),
        (-1, 2),
        (-2, 2),
        (-2, 1),
        (-2, 0),
        (-2, -1),
        (-2, -2),
        (-1, -2),
        (0, -2),
        (1, -2),
        (2, -2),
        (3, -2),
        (3, -1),
    ];

    let expected: Vec<GridCoord> = expected.iter().map(|c| GridCoord::new(c.0, c.1)).collect();
    let actual: Vec<GridCoord> = SpiralCoords::new().take(expected.len()).collect();
    assert_eq!(expected, actual);
}
