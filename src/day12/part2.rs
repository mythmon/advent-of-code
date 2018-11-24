use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;

pub struct Day12Part2;

impl PuzzleRunner for Day12Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D12-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5",
                2,
            )
            .case("Solution", include_str!("input"), 211)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut connections: Vec<(usize, Vec<usize>)> = input
            .lines()
            .map(|l| {
                let parts: Vec<&str> = l.split(" <-> ").collect();
                assert_eq!(parts.len(), 2);
                parts[1]
                    .split(",")
                    .map(|p| p.trim().parse().unwrap())
                    .collect()
            })
            .enumerate()
            .collect();

        let mut group_count = 0;

        while connections.len() > 0 {
            let mut group = HashSet::new();
            group.insert(connections[0].0);

            let mut changed;
            loop {
                changed = false;
                for &(idx, ref connections) in connections.iter() {
                    if group.contains(&idx) {
                        continue;
                    }
                    for c in connections {
                        if group.contains(&c) {
                            changed = true;
                            group.insert(idx);
                            break;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }

            connections.retain(|c| !group.contains(&c.0));
            group_count += 1;
        }

        group_count
    }
}
