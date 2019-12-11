use crate::intcode::IntcodeComputer;
use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

const UP: (isize, isize) = (0, -1);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (1, 0);

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D11-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 2_016)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut pos = (0, 0);
        let mut dir = UP;
        let mut cells: HashMap<(isize, isize), isize> = HashMap::new();
        let mut visited_positions = HashSet::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&0)])
            .done();

        while let Some(new_color) = computer.run_until_output() {
            cells.insert(pos, new_color);
            visited_positions.insert(pos);
            match computer.run_until_output() {
                Some(0) => {
                    dir = match dir {
                        UP => LEFT,
                        DOWN => RIGHT,
                        LEFT => DOWN,
                        RIGHT => UP,
                        _ => panic!("invalid direction {:?}", dir),
                    }
                }
                Some(1) => {
                    dir = match dir {
                        UP => RIGHT,
                        DOWN => LEFT,
                        LEFT => UP,
                        RIGHT => DOWN,
                        _ => panic!("invalid direction {:?}", dir),
                    }
                }
                Some(t) => panic!("invalid turn {}", t),
                None => panic!("Not enough input"),
            }
            pos.0 += dir.0;
            pos.1 += dir.1;
            computer.add_input(*cells.get(&pos).unwrap_or(&0));
        }

        visited_positions.len()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = ();

    fn name(&self) -> String {
        "2019-D11-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), ())
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut pos = (0, 0);
        let mut dir = UP;
        let mut cells: HashMap<(isize, isize), isize> = HashMap::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&1)])
            .done();

        while let Some(new_color) = computer.run_until_output() {
            cells.insert(pos, new_color);
            match computer.run_until_output() {
                Some(0) => {
                    dir = match dir {
                        UP => LEFT,
                        DOWN => RIGHT,
                        LEFT => DOWN,
                        RIGHT => UP,
                        _ => panic!("invalid direction {:?}", dir),
                    }
                }
                Some(1) => {
                    dir = match dir {
                        UP => RIGHT,
                        DOWN => LEFT,
                        LEFT => UP,
                        RIGHT => DOWN,
                        _ => panic!("invalid direction {:?}", dir),
                    }
                }
                Some(t) => panic!("invalid turn {}", t),
                None => panic!("Not enough input"),
            }
            pos.0 += dir.0;
            pos.1 += dir.1;
            computer.add_input(*cells.get(&pos).unwrap_or(&0));
        }

        let min_bound = cells
            .iter()
            .map(|(key, _val)| key)
            .fold((0, 0), |(min_x, min_y), (new_x, new_y)| {
                (cmp::min(min_x, *new_x), cmp::min(min_y, *new_y))
            });
        let max_bound = cells
            .iter()
            .map(|(key, _val)| key)
            .fold((0, 0), |(max_x, max_y), (new_x, new_y)| {
                (cmp::max(max_x, *new_x), cmp::max(max_y, *new_y))
            });

        for y in (min_bound.1)..=(max_bound.1) {
            for x in (min_bound.0)..=(max_bound.0) {
                match cells.get(&(x, y)).unwrap_or(&0) {
                    0 => print!("  "),
                    1 => print!("██"),
                    v => panic!("unexpected cell value {}", v),
                }
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
