use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D12-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
                6_usize,
            )
            .case("Solution", include_str!("input"), 288_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let connections: Vec<Vec<u32>> = input
            .lines()
            .map(|l| {
                let parts: Vec<&str> = l.split(" <-> ").collect();
                assert_eq!(parts.len(), 2);
                parts[1]
                    .split(',')
                    .map(|p| p.trim().parse().unwrap())
                    .collect()
            })
            .collect();

        let mut zero_group = HashSet::new();
        zero_group.insert(0);
        let mut changed;
        loop {
            changed = false;
            for (idx, connections) in connections.iter().enumerate() {
                if zero_group.contains(&(idx as u32)) {
                    continue;
                }
                for c in connections {
                    if zero_group.contains(c) {
                        changed = true;
                        zero_group.insert(idx as u32);
                        break;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        zero_group.len()
    }
}
