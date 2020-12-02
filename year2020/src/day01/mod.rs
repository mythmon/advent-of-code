use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{collections::HashSet, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<u32>;
    type Output = u32;

    fn name(&self) -> String {
        "2020-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case("Example", vec![1_721, 979, 366, 299, 675, 1_456], 514_579)
            .transformed_case("Solution", include_str!("input"), 1_006_176)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        // assumption: there are no duplicates
        let original_len = input.len();
        let numbers: HashSet<_> = input.into_iter().collect();
        assert_eq!(original_len, numbers.len());

        for x in &numbers {
            let y = 2020 - x;
            if *x == y {
                // Can't pair a number with itself
                continue;
            }
            if numbers.contains(&y) {
                return x * y;
            }
        }
        panic!("No answer found");
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<u32>;
    type Output = u32;

    fn name(&self) -> String {
        "2020-D01-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case(
                "Example",
                vec![1_721, 979, 366, 299, 675, 1_456],
                241_861_950,
            )
            .transformed_case("Solution", include_str!("input"), 199_132_160)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        // assumption: there are no duplicates
        let numbers: HashSet<_> = input.clone().into_iter().collect();
        assert_eq!(input.len(), numbers.len());

        for (idx1, x) in input.iter().enumerate() {
            for y in input[(idx1 + 1)..].iter() {
                if x + y >= 2020 {
                    continue;
                }
                let z = 2020 - x - y;
                if z == *x || z == *y {
                    continue;
                }
                if numbers.contains(&z) {
                    return x * y * z;
                }
            }
        }
        panic!("No answer found");
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trimmed_lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
