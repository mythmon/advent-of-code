use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use itertools::Itertools;
use std::{collections::HashMap, iter::Iterator};

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
            .case("Example", "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab", 12_usize) // spell-checker: disable-line
            .case("Solution", include_str!("input"), 5_880_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let (has_double, has_triple) = input
            .trimmed_lines()
            .map(|id| {
                let mut counts = HashMap::new();
                for c in id.chars() {
                    *counts.entry(c).or_insert(0) += 1;
                }
                let has_double = counts.values().any(|count| *count == 2);
                let has_triple = counts.values().any(|count| *count == 3);
                (
                    if has_double { 1 } else { 0 },
                    if has_triple { 1 } else { 0 },
                )
            })
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

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
        let ids = input.trimmed_lines();
        let mut found = None;

        for (id1, id2) in ids.clone().cartesian_product(ids) {
            let mut diff_count = 0;
            for (c1, c2) in id1.chars().zip_eq(id2.chars()) {
                if c1 != c2 {
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

        assert!(found.is_some());
        let found = found.unwrap();
        found
            .0
            .chars()
            .zip_eq(found.1.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect()
    }
}
