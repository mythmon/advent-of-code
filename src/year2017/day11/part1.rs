use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    year2017::day11::{HexDir, HexVec},
};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D11-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "ne,ne,ne", 3_u32)
            .case("Example 2", "ne,ne,sw,sw", 0_u32)
            .case("Example 3", "ne,ne,s,s", 2_u32)
            .case("Example 4", "se,sw,se,sw,sw", 3_u32)
            .case("Correct answer", include_str!("input"), 812_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .trim()
            .split(',')
            .filter_map(|p| p.trim().parse().ok())
            .fold(HexVec::zero(), |a, b: HexDir| a + b)
            .size() as u32
    }
}
