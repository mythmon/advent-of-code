use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = (&'static str, usize);
    type Output = usize;

    fn name(&self) -> String {
        "2017-D22-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", ("..#\n#..\n...", 7), 5_usize)
            .case("Example 2", ("..#\n#..\n...", 70), 41_usize)
            .case("Example 3", ("..#\n#..\n...", 10_000), 5_587_usize)
            .case("Solution", (include_str!("input"), 10_000), 5_259_usize)
            .collect())
    }

    fn run_puzzle((input, iterations): Self::Input) -> Self::Output {
        let mut spores: Sporifica = input.parse().unwrap();

        for _ in 0..iterations {
            spores.tick();
        }

        spores.infections
    }
}

struct Sporifica {
    cells: HashMap<Pos, bool>,
    pos: Pos,
    dir: Dir,
    infections: usize,
}

impl Sporifica {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            pos: Pos::new(0, 0),
            dir: Dir::new(0, -1),
            infections: 0,
        }
    }

    fn tick(&mut self) {
        if *self.cells.get(&self.pos).unwrap_or(&false) {
            self.dir = self.dir.rotate_left();
            self.cells.insert(self.pos, false);
        } else {
            self.dir = self.dir.rotate_right();
            self.cells.insert(self.pos, true);
            self.infections += 1;
        }
        self.pos += self.dir;
    }
}

impl FromStr for Sporifica {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rv = Self::new();

        for (y, line) in input.lines().enumerate() {
            let offset = (line.len() / 2) as i32;
            let y = y as i32;
            for (x, cell) in line.chars().enumerate() {
                let x = x as i32;
                if cell == '#' {
                    let p = Pos::new(x - offset, y - offset);
                    rv.cells.insert(p, true);
                } else {
                    assert_eq!(cell, '.');
                }
            }
        }

        Ok(rv)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Dir> for Pos {
    type Output = Self;

    fn add(self, dir: Dir) -> Self::Output {
        Self::Output {
            x: self.x + dir.x,
            y: self.y + dir.y,
        }
    }
}

impl AddAssign<Dir> for Pos {
    fn add_assign(&mut self, dir: Dir) {
        self.x = (self.x as i32 + dir.x) as i32;
        self.y = (self.y as i32 + dir.y) as i32;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dir {
    x: i32,
    y: i32,
}

impl Dir {
    fn new(x: i32, y: i32) -> Self {
        if x < -1 || x > 1 || y < -1 || y > 1 || (x == 0 && y == 0) {
            panic!(format!("Invalid direction ({}, {})", x, y));
        }
        Self { x, y }
    }

    const fn rotate_left(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    const fn rotate_right(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}
