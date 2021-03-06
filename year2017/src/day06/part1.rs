use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<usize>;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D06-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|raw_input| {
                raw_input
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .case("Example", vec![0, 2, 7, 0], 5_usize)
            .transformed_case("Solution", include_str!("input"), 14_029_usize)?
            .collect())
    }

    fn run_puzzle(mut input: Self::Input) -> Self::Output {
        if input.is_empty() {
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
            for idx in (max_idx + 1)..=(max_idx + remaining) {
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
