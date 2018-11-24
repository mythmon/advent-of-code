use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use crate::day11::{HexDir, HexVec};

pub struct Day11Part2;

impl PuzzleRunner for Day11Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D11-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 1_603)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .split(",")
            .filter_map(|p| p.trim().parse().ok())
            .scan(HexVec::zero(), |a, b: HexDir| {
                *a += b;
                Some(a.size())
            })
            .max()
            .unwrap() as u32
    }
}
