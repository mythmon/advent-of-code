use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<usize>;
    type Output = i32;

    fn name(&self) -> String {
        "2017-D06-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .case("Example", vec![0, 2, 7, 0], 4)
            .transformed_case("Solution", include_str!("input"), 2_765)
            .collect()
    }

    fn run_puzzle(mut input: Self::Input) -> Self::Output {
        if input.is_empty() {
            return 0;
        }

        let mut seen_at = HashMap::new();
        seen_at.insert(input.clone(), 0);

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

            if seen_at.contains_key(&input) {
                return count - seen_at[&input];
            }
            seen_at.insert(input.clone(), count);
        }

        unreachable!();
    }
}
