use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input, 10_000));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str, iterations: usize) -> usize {
    let mut spores: Sporifica = input.parse().unwrap();

    for _ in 0..iterations {
        spores.tick();
    }

    spores.infections
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
        let mut rv = Sporifica::new();

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
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
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
        Self { x: x, y: y }
    }

    fn rotate_left(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    fn rotate_right(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

#[test]
fn test_example() {
    let input = "..#\n#..\n...";
    assert_eq!(puzzle(input, 7), 5);
    assert_eq!(puzzle(input, 70), 41);
    assert_eq!(puzzle(input, 10_000), 5587);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input, 10_000), 5259);
}
