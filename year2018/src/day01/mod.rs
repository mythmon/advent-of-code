use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::collections::HashSet;
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
pub struct Part2;

impl PuzzleRunner for Part2 {
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
        let mut sum = 0_i32;

        let mut seen = HashSet::new();
        seen.insert(sum);

        for n in numbers.iter().cycle() {
            sum += n;
            if seen.contains(&sum) {
                return sum;
            }
            seen.insert(sum);
        }
        unreachable!("Loop only breaks with return");
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trimmed_lines()
        .map(|l| l.trim_start_matches('+'))
        .filter_map(|l| l.parse::<i32>().ok())
        .collect()
}
