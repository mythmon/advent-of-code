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
    type Input = Vec<i32>;
    type Output = i32;

    fn name(&self) -> String {
        "2019-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case("Example 1", vec![12], 2)
            .case("Example 2", vec![14], 2)
            .case("Example 3", vec![1969], 654)
            .case("Example 4", vec![100756], 33583)
            .transformed_case("Solution", include_str!("input"), 3_390_830)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.iter().map(|i| i/3-2).sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<i32>;
    type Output = i32;

    fn name(&self) -> String {
        "2019-D01-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case("Example 2", vec![14], 2)
            .case("Example 3", vec![1969], 966)
            .case("Example 4", vec![100_756], 50_346)
            .transformed_case("Solution", include_str!("input"), 5_083_370)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.iter().map(|i| {
            let mut new_weight = *i;
            let mut total_fuel = 0;
            // weights less than 9 will produce non-positive extra weight
            while new_weight >= 9 {
                new_weight = new_weight / 3 - 2;
                total_fuel += new_weight
            }
            total_fuel
        }).sum()
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trimmed_lines()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect()
}
