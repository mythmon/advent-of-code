use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = u32;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D03-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", 289_325_u32, 295_229_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        GridStressValues::new()
            .skip_while(|v| *v <= input)
            .next()
            .unwrap()
    }
}

struct GridStressValues {
    values: HashMap<GridCoordinate, u32>,
    coords: SpiralCoords,
}

impl GridStressValues {
    fn new() -> Self {
        let mut values = HashMap::new();
        values.insert(GridCoordinate::new(0, 0), 1);
        Self {
            values,
            coords: SpiralCoords::new(),
        }
    }
}

impl Iterator for GridStressValues {
    type Item = u32;

    // I can't figure out how to use the entry API since the value to be
    // inserted depends on borrowing `self` again.
    #[allow(clippy::map_entry)]
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.coords.next().unwrap();
        if self.values.contains_key(&p) {
            Some(self.values[&p])
        } else {
            let s = p
                .neighbors()
                .iter()
                .map(|n| self.values.get(n).unwrap_or(&0))
                .sum();
            self.values.insert(p, s);
            Some(s)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridCoordinate {
    x: i32,
    y: i32,
}

impl GridCoordinate {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

impl Add<Direction> for GridCoordinate {
    type Output = Self;

    fn add(self, dir: Direction) -> Self::Output {
        Self::Output {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl AddAssign<Direction> for GridCoordinate {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Debug)]
struct SpiralCoords {
    dir: Direction,
    pos: GridCoordinate,
    stride: u32,
    idx: u32,
    parity: bool,
}

impl SpiralCoords {
    fn new() -> Self {
        Self {
            dir: Direction::new(1, 0),
            pos: GridCoordinate::new(0, 0),
            stride: 1,
            idx: 0,
            parity: false,
        }
    }
}

impl Iterator for SpiralCoords {
    type Item = GridCoordinate;

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
        Self { x, y }
    }

    const fn rotate90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

#[test]
fn test_grid_stress_values() {
    let expected = vec![
        1, 1, 2, 4, 5, 10, 11, 23, 25, 26, 54, 57, 59, 122, 133, 142, 147, 304, 330, 351, 362, 747,
        806,
    ];
    let actual: Vec<u32> = GridStressValues::new().take(expected.len()).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_spiral_coords() {
    #[rustfmt::skip]
    let expected = vec![
        (0, 0), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1),
        (1, -1), (2, -1), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (-1, 2),
        (-2, 2), (-2, 1), (-2, 0), (-2, -1), (-2, -2), (-1, -2), (0, -2),
        (1, -2), (2, -2), (3, -2), (3, -1),
    ];

    let expected: Vec<GridCoordinate> = expected
        .iter()
        .map(|c| GridCoordinate::new(c.0, c.1))
        .collect();
    let actual: Vec<GridCoordinate> = SpiralCoords::new().take(expected.len()).collect();
    assert_eq!(expected, actual);
}
