use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

#[derive(Debug)]
pub struct Day01Part1;

impl PuzzleRunner for Day01Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "1122", 3)
            .case("Example 2", "1111", 4)
            .case("Example 3", "1234", 0)
            .case("Example 4", "91212129", 9)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut input: Vec<u32> = input
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let first = input.first().unwrap().clone();
        input.push(first);

        let mut sum = 0;
        for (&a, &b) in input.iter().zip(&input[1..]) {
            if a == b {
                sum += a;
            }
        }
        sum
    }
}
