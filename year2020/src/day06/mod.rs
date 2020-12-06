use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::{
    collections::{HashMap, HashSet},
    iter::Iterator,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D06-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc! {"
                abc

                a
                b
                c

                ab
                ac

                a
                a
                a
                a

                b
            "},
                11,
            )
            .case("Solution", include_str!("input"), 6_726)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .paragraphs()
            .map(|group| {
                group
                    .chars()
                    .filter(|c| ('a'..='z').contains(c))
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D06-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc! {"
                abc

                a
                b
                c

                ab
                ac

                a
                a
                a
                a

                b
            "},
                6,
            )
            .case("Solution", include_str!("input"), 3_316)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .paragraphs()
            .map(|group| {
                let group_size = group.lines().count();
                group
                    .chars()
                    .filter(|c| ('a'..='z').contains(c))
                    .fold(HashMap::new(), |mut counts, c| {
                        *counts.entry(c).or_insert(0) += 1;
                        counts
                    })
                    .values()
                    .filter(|v| **v == group_size)
                    .count()
            })
            .sum()
    }
}
