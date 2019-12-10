use crate::intcode::IntcodeComputer;
use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D09-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 2_453_265_701)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(input).with_input(vec![1]).done();
        computer.run_to_end();
        *computer.output.last().expect("no output")
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D09-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 80_805)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(input).with_input(vec![2]).done();
        computer.run_to_end();
        *computer.output.last().expect("no output")
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
