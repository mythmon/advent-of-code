use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;

pub struct Day06Part1;

impl PuzzleRunner for Day06Part1 {
    type Input = Vec<usize>;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D06-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|raw_input| {
                raw_input
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .case("Example", vec![0, 2, 7, 0], 5)
            .transformed_case("Solution", include_str!("input"), 14_029)
            .collect()
    }

    fn run_puzzle(mut input: Self::Input) -> Self::Output {
        if input.len() == 0 {
            return 0;
        }

        let mut seen = HashSet::new();
        seen.insert(input.clone());

        for count in 1.. {
            let mut max_idx = 0;
            let mut max = input[0];

            for (idx, &item) in input.iter().enumerate() {
                if item > max {
                    max = item;
                    max_idx = idx;
                }
            }

            let remaining = max;
            input[max_idx] = 0;
            for idx in (max_idx + 1)..(max_idx + remaining + 1) {
                let wrapped_idx = idx % input.len();
                input[wrapped_idx] += 1;
            }

            if seen.contains(&input) {
                return count;
            }
            seen.insert(input.clone());
        }

        unreachable!();
    }
}
