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
        "2019-D02-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 3_790_645)
            .collect()
    }

    fn run_puzzle(mut program: Self::Input) -> Self::Output {
        program[1] = 12;
        program[2] = 2;
        let mut computer = IntcodeComputer::build(program).done();
        computer.run_to_end();
        computer.read_mem(0)
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D02-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 6577)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        for a in 0..100 {
            for b in 0..100 {
                let mut modified_program = program.clone();
                modified_program[1] = a;
                modified_program[2] = b;
                let mut computer = IntcodeComputer::build(modified_program).done();
                computer.run_to_end();

                if computer.read_mem(0) == 19_690_720 {
                    return 100 * a + b;
                }
            }
        }
        panic!("No answer found");
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    let line = input.trimmed_lines().next().unwrap();
    line.split(',')
        .filter_map(|n| n.parse::<isize>().ok())
        .collect()
}
