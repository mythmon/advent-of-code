use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = i32;

    fn name(&self) -> String {
        "2015-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
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
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unexpected char"),
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

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", ")", 1)
            .case("Example 2", "()())", 5)
            .case("Solution", include_str!("input"), 1783)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .trim()
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unexpected char"),
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
