use crate::intcode::{IntcodeComputer, PauseReason};
use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::{cmp::Ordering, collections::VecDeque, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D13-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 361)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(input).done();
        computer.run_to_end();
        computer
            .output
            .into_iter()
            .skip(2)
            .step_by(3)
            .filter(|c| *c == 2)
            .count()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D13-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 17_590)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut computer = IntcodeComputer::build(input).done();
        computer.write_mem(0, 2);

        let mut score = 0;
        let mut paddle_x = 0;
        let mut ball_x = 0;
        let mut output = VecDeque::new();

        loop {
            match computer.run_until_io() {
                PauseReason::Output(v) => {
                    output.push_back(v);
                    while output.len() >= 3 {
                        let x = output.pop_front().unwrap();
                        let y = output.pop_front().unwrap();
                        let symbol = output.pop_front().unwrap();
                        match (x, y, symbol) {
                            (-1, _, s) => score = s,
                            (x, _, 3) => paddle_x = x,
                            (x, _, 4) => ball_x = x,
                            _ => {}
                        };
                    }
                }
                PauseReason::Input => {
                    let x = match ball_x.cmp(&paddle_x) {
                        Ordering::Greater => 1,
                        Ordering::Equal => 0,
                        Ordering::Less => -1,
                    };
                    computer.add_input(x);
                }
                PauseReason::Halt => {
                    break;
                }
            }
        }

        score
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}
