use crate::day11::{HexDir, HexVec};
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D11-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 1_603_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .split(',')
            .filter_map(|p| p.trim().parse().ok())
            .scan(HexVec::zero(), |a, b: HexDir| {
                *a += b;
                Some(a.size())
            })
            .max()
            .unwrap() as u32
    }
}
