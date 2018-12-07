use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::{collections::HashSet, iter::Iterator};

#[derive(Debug)]
pub struct Day05Part1;

impl PuzzleRunner for Day05Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D05-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "aA", 0usize)
            .case("Example 2", "abBA", 0usize)
            .case("Example 3", "abAB", 4usize)
            .case("Example 4", "aabAAB", 6usize)
            .case("Example 5", "dabAcCaCBAcCcaDA", 10usize)
            .case("Test with newlines", "aADbBdcC\n", 0usize)
            .case("Solution", include_str!("input"), 9_078usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        collapse_polymer(input.trim()).len()
    }
}

fn collapse_polymer(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();

    loop {
        let mut any_changes = false;
        let mut next = Vec::new();
        let mut idx = 0;
        while idx < chars.len() {
            if idx == chars.len() - 1 {
                // last char, push it and increment
                next.push(chars[idx]);
                idx += 1;
            } else {
                // compare this and next
                let first = chars[idx];
                let second = chars[idx + 1];
                // if the chars are the same ignoring case, but different including case
                if first.eq_ignore_ascii_case(&second) && first != second {
                    // don't include the char
                    any_changes = true;
                    // skip over this and the next char
                    idx += 2;
                } else {
                    next.push(first);
                    idx += 1;
                }
            }
        }
        chars = next;
        if !any_changes {
            break;
        }
    }
    chars.iter().collect()
}

#[derive(Debug)]
pub struct Day05Part2;

impl PuzzleRunner for Day05Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D05-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "dabAcCaCBAcCcaDA", 4usize)
            .case("Solution", include_str!("input"), 5_698usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let input = input.trim();
        let unit_types: HashSet<_> = input.to_lowercase().chars().collect();
        unit_types
            .into_iter()
            .map(|unit| {
                let candidate = input.replace(unit, "").replace(unit.to_ascii_uppercase(), "");
                let collapsed = collapse_polymer(&candidate);
                collapsed.len()
            })
            .min()
            .unwrap()
    }
}