use crate::day10::KnotHash;
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = String;

    fn name(&self) -> String {
        "2017-D10-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example 1",
                "",
                "a2582a3a0e66e6e86e3812dcb672a272".to_owned(),
            )
            .case(
                "Example 2",
                "AoC 2017",
                "33efeb34ea91902bb2f59c9920caa6cd".to_owned(),
            )
            .case(
                "Example 3",
                "1,2,3",
                "3efbe78a8d82f29979031a4aa0b16a9d".to_owned(),
            )
            .case(
                "Example 4",
                "1,2,4",
                "63960835bcdc130f0b66d7ff4f6a5a8e".to_owned(),
            )
            .case(
                "Solution",
                include_str!("input").trim(),
                "70b856a24d586194331398c7fcfa0aaf".to_owned(),
            )
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        KnotHash::new(input).hex()
    }
}
