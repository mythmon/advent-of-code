use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::str::FromStr;
use std::{cmp, fmt};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = (&'static str, usize);
    type Output = usize;

    fn name(&self) -> String {
        "2017-D22-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", ("..#\n#..\n...", 100), 26_usize)
            .case("Example 2", ("..#\n#..\n...", 10_000_000), 2_511_944_usize)
            .case(
                "Solution",
                (include_str!("input"), 10_000_000),
                2_511_722_usize,
            )
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

#[derive(Debug, Clone, Copy)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl fmt::Display for NodeState {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NodeState::Clean => write!(formatter, ".")?,
            NodeState::Weakened => write!(formatter, "W")?,
            NodeState::Infected => write!(formatter, "#")?,
            NodeState::Flagged => write!(formatter, "F")?,
        };
        Ok(())
    }
}

struct Sporifica {
    cells: HashMap<Pos, NodeState>,
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
        match *self.cells.get(&self.pos).unwrap_or(&NodeState::Clean) {
            NodeState::Clean => {
                self.dir = self.dir.rotate_left();
                self.cells.insert(self.pos, NodeState::Weakened);
            }
            NodeState::Weakened => {
                self.infections += 1;
                self.cells.insert(self.pos, NodeState::Infected);
            }
            NodeState::Infected => {
                self.dir = self.dir.rotate_right();
                self.cells.insert(self.pos, NodeState::Flagged);
            }
            NodeState::Flagged => {
                self.dir = self.dir.reverse();
                self.cells.insert(self.pos, NodeState::Clean);
            }
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
                    rv.cells.insert(p, NodeState::Infected);
                } else {
                    assert_eq!(cell, '.');
                }
            }
        }

        Ok(rv)
    }
}

impl fmt::Display for Sporifica {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min_x = None;
        let mut max_x = None;
        let mut min_y = None;
        let mut max_y = None;

        for pos in self.cells.keys() {
            min_x = Some(cmp::min(pos.x, min_x.unwrap_or(pos.x)));
            max_x = Some(cmp::min(pos.x, max_x.unwrap_or(pos.x)));
            min_y = Some(cmp::min(pos.x, min_y.unwrap_or(pos.y)));
            max_y = Some(cmp::min(pos.x, max_y.unwrap_or(pos.y)));
        }

        let min_x = min_x.unwrap_or(0);
        let max_x = max_x.unwrap_or(0);
        let min_y = min_y.unwrap_or(0);
        let max_y = max_y.unwrap_or(0);

        for y in (min_y - 1)..(max_y + 4) {
            for x in (min_x - 1)..(max_x + 4) {
                let pos = Pos::new(x, y);
                let state = self.cells.get(&pos).unwrap_or(&NodeState::Clean);
                if self.pos == pos {
                    write!(formatter, "[{}]", state)?;
                } else {
                    write!(formatter, " {} ", state)?;
                }
            }
            writeln!(formatter)?;
        }

        Ok(())
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
            x: self.y,
            y: -self.x,
        }
    }

    const fn rotate_right(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    const fn reverse(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[test]
fn test_graphical() {
    let input = "..#\n#..\n...";
    let mut spores: Sporifica = input.parse().unwrap();

    println!("{}", spores);

    for _ in 0..7 {
        spores.tick();
        println!("{}", spores);
    }

    assert_eq!(spores.infections, 1);
}
