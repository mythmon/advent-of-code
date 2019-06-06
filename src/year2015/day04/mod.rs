#![allow(clippy::maybe_infinite_iter)]
use crate::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use rayon::prelude::*;
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn find_leading_zeroes(input: &'static str, n: usize) -> u32 {
    let input = input.trim();
    let section_size = 10_000;
    let chunk_size = 100;
    let num_chunks = section_size / chunk_size;

    let needle: String = std::iter::repeat("0").take(n).collect();

    for section in 0.. {
        let base = section_size * section;
        let starting_points: Vec<_> = (0..num_chunks).map(|n| base + n * chunk_size + 1).collect();
        let founds: Vec<u32> = starting_points
            .par_iter()
            .filter_map(|&starting_point| {
                for nonce in starting_point..(starting_point + chunk_size) {
                    let input = format!("{}{}", input, nonce);
                    let hash = md5::compute(input.as_bytes());
                    let hex_hash = hex::encode(hash.0);
                    if hex_hash.starts_with(&needle) {
                        return Some(nonce);
                    }
                }
                None
            })
            .collect();

        if !founds.is_empty() {
            return *founds.iter().min().unwrap();
        }
    }

    unreachable!();
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2015-D04-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "abcdef", 609_043)
            .case("Example", "pqrstuv", 1_048_970)
            .case("Solution", include_str!("input"), 346_386)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        find_leading_zeroes(input, 5)
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2015-D04-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 9_958_218)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        find_leading_zeroes(input, 6)
    }
}
