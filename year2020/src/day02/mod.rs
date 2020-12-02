use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(Part1),
        Box::new(Part2),
    ]
}

#[derive(PartialEq, Debug, Clone)]
pub struct Input {
    first: usize,
    second: usize,
    letter: char,
    password: String,
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Input>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D02-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Example 1a", "1-3 a: abcde", 1)
            .transformed_case("Example 1b", "1-3 b: cdefg", 0)
            .transformed_case("Example 1c", "2-9 c: ccccccccc", 1)
            .transformed_case("Solution", include_str!("input"), 580)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.into_iter()
        .filter(|entry| {
            let count = entry.password.chars().filter(|c| *c == entry.letter).count();
            count >= entry.first && count <= entry.second
        })
        .count()
    }
}
#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Input>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D02-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Example 1a", "1-3 a: abcde", 1)
            .transformed_case("Example 1b", "1-3 b: cdefg", 0)
            .transformed_case("Example 1c", "2-9 c: ccccccccc", 0)
            .transformed_case("Solution", include_str!("input"), 611)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.into_iter()
        .filter(|entry| {
            let first_match = entry.password.chars().nth(entry.first - 1)== Some(entry.letter);
            let second_match = entry.password.chars().nth(entry.second - 1) == Some(entry.letter);
            first_match ^ second_match
        })
        .count()
    }
}

fn parse_input(input: &str) -> Vec<Input> {
    input
        .trimmed_lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let range: Vec<&str> = parts[0].split('-').collect();
            let letter = parts[1].chars().next().unwrap();
            Input {
                first: range[0].parse().expect("Couldn't parse min"),
                second: range[1].parse().expect("Couldn't parse max"),
                letter,
                password: parts[2].to_string(),
            }
        })
        .collect()
}
