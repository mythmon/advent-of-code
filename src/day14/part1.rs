use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use crate::day10::KnotHash;

#[derive(Debug)]
pub struct Day14Part1;

impl PuzzleRunner for Day14Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D14-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "flqrgnkx", 8108)
            .case("Solution", include_str!("input"), 8148)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        (0..128)
            .map(|row| format!("{}-{}", input.trim(), row))
            .flat_map(|row_input| KnotHash::new(&row_input).dense())
            .map(|hash_part| hash_part.count_ones())
            .sum()
    }
}
