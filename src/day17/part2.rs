use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day17Part2;

impl PuzzleRunner for Day17Part2 {
    type Input = usize;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D17-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.trim().parse().unwrap())
            .transformed_case("Solution", include_str!("input"), 11_162_912)
            .collect()
    }

    fn run_puzzle(step: Self::Input) -> Self::Output {
        let max = 50_000_000;

        let mut val_after_0 = None;
        let mut pos = 0;
        let mut current_length = 1;

        for i in 1..max {
            pos = (pos + step) % current_length;
            if pos == 0 {
                val_after_0 = Some(i);
            }
            current_length += 1;
            pos += 1;
        }

        val_after_0.unwrap()
    }
}
