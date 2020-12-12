use crate::intcode::{IntcodeComputer, PauseReason};
use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    grid_letters::{Recognizer, ALPHABET_2019_D11},
    twodee::{Dir4, Point, Turn},
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

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 2_016)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut pos = Point::zero();
        let mut dir = Dir4::Up;
        let mut cells: HashMap<Point<isize>, isize> = HashMap::new();
        let mut visited_positions = HashSet::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&0)])
            .done();

        while let PauseReason::Output(new_color) = computer.run_until_io() {
            cells.insert(pos, new_color);
            visited_positions.insert(pos);
            match computer.run_until_io() {
                PauseReason::Output(0) => dir *= Turn::Ccw,
                PauseReason::Output(1) => dir *= Turn::Cw,
                PauseReason::Output(t) => panic!("invalid turn {}", t),
                PauseReason::Input => panic!("Not enough input"),
                PauseReason::Halt => panic!("Computer ended early"),
            };
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
    type Output = String;

    fn name(&self) -> String {
        "2019-D11-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), "RAPRCBPH".to_string())?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        #[allow(clippy::missing_const_for_fn)] // It can't be since it consumes its inputs
        fn first<T, U>((a, _b): (T, U)) -> T {
            a
        }

        let mut pos = Point::zero();
        let mut dir = Dir4::Up;
        let mut cells: HashMap<Point<_>, isize> = HashMap::new();
        let mut computer = IntcodeComputer::build(input)
            .with_input(vec![*cells.get(&pos).unwrap_or(&1)])
            .done();

        while let PauseReason::Output(new_color) = computer.run_until_io() {
            cells.insert(pos, new_color);
            match computer.run_until_io() {
                PauseReason::Output(0) => dir *= Turn::Ccw,
                PauseReason::Output(1) => dir *= Turn::Cw,
                PauseReason::Output(t) => panic!("invalid turn {}", t),
                PauseReason::Input => panic!("Not enough input"),
                PauseReason::Halt => panic!("Unexpected halt"),
            }
            pos += dir;
            computer.add_input(*cells.get(&pos).unwrap_or(&0));
        }

        let top_left: Point<isize> = cells.iter().map(first).fold(Point::zero(), |min, new| {
            Point::new(cmp::min(min.x, new.x), cmp::min(min.y, new.y))
        });
        let bottom_right = cells.iter().map(first).fold(Point::zero(), |max, new| {
            Point::new(cmp::max(max.x, new.x), cmp::max(max.y, new.y))
        });

        let output_height = (bottom_right.y - top_left.y).abs() as usize + 1;
        let output_width = (bottom_right.x - top_left.x).abs() as usize + 1;
        let mut output = String::with_capacity(output_height * output_width);

        for y in (top_left.y)..=(bottom_right.y) {
            for x in (top_left.x)..=(bottom_right.x) {
                output.extend(match cells.get(&Point::new(x, y)).unwrap_or(&0) {
                    0 => "  ".chars(),
                    1 => "██".chars(),
                    v => panic!("unexpected cell value {}", v),
                });
            }
            output.push('\n');
        }

        Recognizer::new(ALPHABET_2019_D11).parse(&output)
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
