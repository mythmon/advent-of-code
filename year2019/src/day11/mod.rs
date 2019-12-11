use crate::intcode::IntcodeComputer;
use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    twodee::{Dir, Point, Turn},
};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

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
        let mut pos = Point::zero();
        let mut dir = Dir::Up;
        let mut cells: HashMap<Point<isize>, isize> = HashMap::new();
        let mut visited_positions = HashSet::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&0)])
            .done();

        while let Some(new_color) = computer.run_until_output() {
            cells.insert(pos, new_color);
            visited_positions.insert(pos);
            match computer.run_until_output() {
                Some(0) => dir *= Turn::Ccw,
                Some(1) => dir *= Turn::Cw,
                Some(t) => panic!("invalid turn {}", t),
                None => panic!("Not enough input"),
            }
            pos += dir;
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
        let mut pos = Point::zero();
        let mut dir = Dir::Up;
        let mut cells: HashMap<Point<_>, isize> = HashMap::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&1)])
            .done();

        while let Some(new_color) = computer.run_until_output() {
            cells.insert(pos, new_color);
            match computer.run_until_output() {
                Some(0) => dir *= Turn::Ccw,
                Some(1) => dir *= Turn::Cw,
                Some(t) => panic!("invalid turn {}", t),
                None => panic!("Not enough input"),
            }
            pos += dir;
            computer.add_input(*cells.get(&pos).unwrap_or(&0));
        }

        fn first<T, U>((a, _b): (T, U)) -> T {
            a
        }

        let top_left: Point<isize> = cells.iter().map(first).fold(Point::zero(), |min, new| {
            Point::new(cmp::min(min.x, new.x), cmp::min(min.y, new.y))
        });
        let bottom_right = cells.iter().map(first).fold(Point::zero(), |max, new| {
            Point::new(cmp::max(max.x, new.x), cmp::max(max.y, new.y))
        });

        for y in (top_left.y)..=(bottom_right.y) {
            for x in (top_left.x)..=(bottom_right.x) {
                match cells.get(&Point::new(x, y)).unwrap_or(&0) {
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
