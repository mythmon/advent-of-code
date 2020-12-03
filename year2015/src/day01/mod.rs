use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = i32;

    fn name(&self) -> String {
        "2015-D01-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "(())", 0)
            .case("Example 2", "()()", 0)
            .case("Example 3", "(((", 3)
            .case("Example 4", "(()(()(", 3)
            .case("Example 5", "))(((((", 3)
            .case("Example 6", "())", -1)
            .case("Example 7", "))(", -1)
            .case("Example 8", ")))", -3)
            .case("Example 9", ")())())", -3)
            .case("Solution", include_str!("input"), 232)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2015-D01-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", ")", 1)
            .case("Example 2", "()())", 5)
            .case("Solution", include_str!("input"), 1783)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .scan(0, |floor, delta| {
                *floor += delta;
                Some(*floor)
            })
            .take_while(|&floor| floor >= 0)
            .count()
            + 1
    }
}
