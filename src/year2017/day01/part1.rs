use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D01-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "1122", 3_u32)
            .case("Example 2", "1111", 4_u32)
            .case("Example 3", "1234", 0_u32)
            .case("Example 4", "91212129", 9_u32)
            .case("Solution", include_str!("input"), 1_141_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut input: Vec<u32> = input
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        input.push(*input.first().unwrap());

        let mut sum = 0;
        for (&a, &b) in input.iter().zip(&input[1..]) {
            if a == b {
                sum += a;
            }
        }
        sum
    }
}
