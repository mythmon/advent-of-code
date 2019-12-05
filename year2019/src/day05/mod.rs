use crate::intcode::IntcodeComputer;
use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
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
        "2019-D05-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 7_566_643)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(program).with_input(vec![1]).done();
        computer.run_to_end();
        *computer.output.last().unwrap()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D05-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 9_265_694)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(program).with_input(vec![5]).done();
        computer.run_to_end();
        *computer.output.last().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    let line = input.trimmed_lines().next().unwrap();
    line.split(',')
        .filter_map(|n| n.parse::<isize>().ok())
        .collect()
}
