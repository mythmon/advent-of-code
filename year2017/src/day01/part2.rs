use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D01-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, &'static str, u32>::build_set()
            .case("Example 1", "1212", 6_u32)
            .case("Example 2", "1221", 0_u32)
            .case("Example 3", "123425", 4_u32)
            .case("Example 4", "123123", 12_u32)
            .case("Example 5", "12131415", 4_u32)
            .case("Solution", include_str!("input"), 950_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let input: Vec<u32> = input
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let offset_input = input.iter().cycle().skip(input.len() / 2);

        let mut sum: u32 = 0;
        for (&a, &b) in input.iter().zip(offset_input) {
            if a == b {
                sum += a;
            }
        }
        sum
    }
}
