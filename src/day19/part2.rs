#![feature(range_contains)]
use std::ops::{Add, AddAssign};

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let max_width = grid.iter().map(|row| row.len()).max().unwrap();
    for row in grid.iter_mut() {
        row.resize(max_width, ' ');
    }

    let mut pos = None;
    for (idx, cell) in grid[0].iter().enumerate() {
        if *cell == '|' {
            pos = Some(Pos::new(idx as i32, 0));
            break;
        }
    }
    let mut pos = pos.unwrap();
    let mut dir = Dir::new(0, 1);

    let mut steps = 0;
    loop {
        pos += dir;
        steps += 1;
        let c = grid[pos.y as usize][pos.x as usize];
        match c {
            '|' | '-' => (),
            '+' => {
                let right = dir.clone().rotate90();
                let left = right.clone().rotate90().rotate90();

                let mut found = false;

                for new_dir in [left, right].into_iter() {
                    let new_pos = pos + *new_dir;
                    if new_pos.x >= 0 && new_pos.x < max_width as i32 && new_pos.y >= 0 && new_pos.y < grid.len() as i32 {
                        let c = grid[new_pos.y as usize][new_pos.x as usize];
                        if c != ' ' {
                            dir = *new_dir;
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    panic!("couldn't find anywhere to go");
                }
            }
            c if ('A' ..= 'Z').contains(&c) => (),
            ' ' => break,
            c => panic!(format!("unexpected character '{}'", c)),
        }
    }

    steps
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

    fn rotate90(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}


#[test]
fn test_example() {
    let input: &'static str = include_str!("example");
    assert_eq!(puzzle(input), 38);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    // assert_eq!(puzzle(input), 42);
}
