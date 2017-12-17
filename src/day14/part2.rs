#![feature(range_contains)]

extern crate advent;

use std::collections::HashSet;
use std::fmt;

use advent::day10::KnotHash;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input.trim()
}

fn puzzle(input: &str) -> usize {
    let size = 128;
    let grid = KnotGrid::from(input);
    let mut groups_found = 0;
    let mut todo = Vec::with_capacity(size * size);
    let mut done = HashSet::with_capacity(size * size);

    for x in 0..size {
        for y in 0..size {
            todo.push((x, y));
        }
    }

    for pos in todo.into_iter() {
        if done.insert(pos) && grid.get(pos) {
            groups_found += 1;
            let mut group_todo = Vec::from(pos.neighbors(size));

            while group_todo.len() > 0 {
                let group_pos = group_todo.pop().unwrap();
                done.insert(group_pos);
                if grid.get(group_pos) {
                    let mut ns: Vec<(usize, usize)> = group_pos
                        .neighbors(size)
                        .into_iter()
                        .filter(|n| !done.contains(n))
                        .collect();
                    group_todo.append(&mut ns);
                }
            }
        }
    }

    groups_found
}

struct KnotGrid {
    hash_rows: Vec<Vec<usize>>,
}

impl KnotGrid {
    fn from<T: fmt::Display>(input: T) -> Self {
        Self {
            hash_rows: (0..128)
                .map(|row| format!("{}-{}", input, row))
                .map(|row_input| KnotHash::new(&row_input).dense())
                .collect(),
        }
    }

    fn get<T>(&self, index: T) -> bool
    where
        T: Into<(usize, usize)>,
    {
        let (row_idx, col_idx) = index.into();
        let row = &self.hash_rows[row_idx];
        let cell = row[col_idx / 8];
        assert!((0..256).contains(cell));
        let cell_idx = 7 - (col_idx % 8);
        cell >> cell_idx & 1 == 1
    }
}

impl fmt::Display for KnotGrid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for row in self.hash_rows.iter() {
            for cell in row.iter() {
                write!(formatter, "{:08b}", cell)?;
            }
            write!(formatter, "\n")?;
        }

        Ok(())
    }
}

trait Pos: Sized {
    fn neighbors(&self, limit: usize) -> Vec<Self>;
}

impl Pos for (usize, usize) {
    fn neighbors(&self, limit: usize) -> Vec<Self> {
        let l = limit - 1;
        let mut rv = Vec::with_capacity(4);

        if self.0 > 0 {
            rv.push((self.0 - 1, self.1));
        }
        if self.0 < l {
            rv.push((self.0 + 1, self.1));
        }
        if self.1 > 0 {
            rv.push((self.0, self.1 - 1));
        }
        if self.1 < l {
            rv.push((self.0, self.1 + 1));
        }

        rv
    }
}

#[test]
fn test_example() {
    let input = "flqrgnkx";
    assert_eq!(puzzle(input), 1242);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    // assert_eq!(puzzle(input), 8148);
    assert!(puzzle(input) < 1265);
    assert!(puzzle(input) > 1154);
}

#[test]
fn test_neighbors() {
    assert_eq!((0, 0).neighbors(3), vec![(1, 0), (0, 1)]);
    assert_eq!((1, 0).neighbors(3), vec![(0, 0), (2, 0), (1, 1)]);
    assert_eq!((2, 0).neighbors(3), vec![(1, 0), (2, 1)]);
    assert_eq!((1, 1).neighbors(3), vec![(0, 1), (2, 1), (1, 0), (1, 2)]);
}

#[test]
fn test_knotgrid_get() {
    let mut rows = Vec::with_capacity(128);
    for _ in 0..128 {
        let mut row = Vec::with_capacity(16);
        row.resize(16, 0b10101010);
        rows.push(row);
    }
    let grid = KnotGrid { hash_rows: rows };
    assert!(grid.get((0, 0)));
    assert!(grid.get((1, 0)));
    assert!(!grid.get((0, 1)));
    assert!(!grid.get((1, 1)));
}