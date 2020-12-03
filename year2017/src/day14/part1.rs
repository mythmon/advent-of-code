use crate::day10::KnotHash;
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D14-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "flqrgnkx", 8_108_u32)
            .case("Solution", include_str!("input"), 8_148_u32)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        (0..128)
            .map(|row| format!("{}-{}", input.trim(), row))
            .flat_map(|row_input| KnotHash::new(&row_input).dense())
            .map(usize::count_ones)
            .sum()
    }
}
