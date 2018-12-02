use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Day02Part1;

impl PuzzleRunner for Day02Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D02-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab", 12) // spell-checker: disable-line
            .case("Solution", include_str!("input"), 5880)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let ids: Vec<_> = input.lines().filter(|l| *l != "").collect();

        let has_double = ids
            .iter()
            .filter(|id| {
                let mut counts = HashMap::new();
                for c in id.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }

                counts.values().any(|count| *count == 2)
            })
            .count();

        let has_triple = ids
            .iter()
            .filter(|id| {
                let mut counts = HashMap::new();
                for c in id.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
                counts.values().any(|count| *count == 3)
            })
            .count();

        has_double * has_triple
    }
}

#[derive(Debug)]
pub struct Day02Part2;

impl PuzzleRunner for Day02Part2 {
    type Input = &'static str;
    type Output = String;

    fn name(&self) -> String {
        "2018-D02-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            // spell-checker: disable
            .case("Example", "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n", "fgij".to_owned())
            .case("Solution", include_str!("input"), "tiwcdpbseqhxryfmgkvjujvza".to_owned())
            // spell-checker: enable
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let ids: Vec<_> = input.lines().filter(|l| *l != "").collect();

        let mut found = None;

        // todo this would panic if there was no answer
        for (idx, id1) in ids.iter().enumerate() {
            for id2 in &ids[idx + 1..] {
                assert_eq!(id1.len(), id2.len());
                let mut diff_count = 0;
                for c_idx in 0..id1.len() {
                    if id1[c_idx..=c_idx] != id2[c_idx..=c_idx] {
                        diff_count += 1;
                        if diff_count > 1 {
                            break;
                        }
                    }
                }
                if diff_count == 1 {
                    found = Some((id1, id2));
                }
            }
        }

        assert!(found.is_some());
        let found = found.unwrap();
        found
            .0
            .chars()
            .zip(found.1.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect()
    }
}
