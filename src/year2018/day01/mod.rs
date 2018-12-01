use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Day01Part1;

impl PuzzleRunner for Day01Part1 {
    type Input = &'static str;
    type Output = i32;

    fn name(&self) -> String {
        "2018-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "+1\n-2\n+3\n+1\n", 3)
            .case("Example 2", "+1\n+1\n+1\n", 3)
            .case("Example 3", "+1\n+1\n-2\n", 0)
            .case("Example 4", "-1\n-2\n-3\n", -6)
            .case("Solution", include_str!("input"), 502)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        parse_input(input).iter().sum()
    }
}

#[derive(Debug)]
pub struct Day01Part2;

impl PuzzleRunner for Day01Part2 {
    type Input = &'static str;
    type Output = i32;

    fn name(&self) -> String {
        "2018-D01-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 2", "+1\n-1\n", 0)
            .case("Example 3", "+3\n+3\n+4\n-2\n-4\n", 10)
            .case("Example 4", "-6\n+3\n+8\n+5\n-6\n", 5)
            .case("Example 5", "+7\n+7\n-2\n-7\n-4\n", 14)
            .case("Solution", include_str!("input"), 71_961)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let numbers = parse_input(input);
        let mut sum = 0i32;

        let mut seen = HashSet::new();
        seen.insert(sum);

        for n in numbers.iter().cycle() {
            sum += n;
            if seen.contains(&sum) {
                return sum;
            }
            seen.insert(sum);
        }
        unreachable!();
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| *l != "")
        .map(|l| l.trim_start_matches("+"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}