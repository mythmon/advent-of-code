use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use itertools::Itertools;
use std::{collections::HashSet, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = (u32, u32);
    type Output = usize;

    fn name(&self) -> String {
        "2019-D04-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", (111_111, 111_111), 1)
            .case("Example 2", (223_450, 223_450), 0)
            .case("Example 3", (123_789, 123_789), 0)
            .case("Debug 1", (100_000, 100_100), 0)
            .case("Solution", (367_479, 893_698), 495)
            .collect()
    }

    fn run_puzzle((low, high): Self::Input) -> Self::Output {
        (low..=high)
            .map(|n| {
                n.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .filter(|digits: &Vec<u32>| digits.windows(2).all(|pair| pair[0] <= pair[1]))
            .filter(|digits| digits.windows(2).any(|pair| pair[0] == pair[1]))
            .count()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = (u32, u32);
    type Output = usize;

    fn name(&self) -> String {
        "2019-D04-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", (11_22_33, 11_22_33), 1)
            .case("Example 2", (123_444, 123_444), 0)
            .case("Example 3", (111_122, 111_122), 1)
            .case("Solution", (367_479, 893_698), 305)
            .collect()
    }

    fn run_puzzle((low, high): Self::Input) -> Self::Output {
        (low..=high)
            .map(|n| {
                n.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .filter(|digits: &Vec<u32>| digits.iter().tuple_windows().all(|(a, b)| a <= b))
            .filter(|digits| {
                let double_indexes: HashSet<_> = digits
                    .iter()
                    .tuple_windows()
                    .enumerate()
                    .filter(|(_, (a, b))| a == b)
                    .map(|(left_index, _)| left_index)
                    .collect();
                let indexes_in_triples: HashSet<_> = digits
                    .iter()
                    .tuple_windows()
                    .enumerate()
                    .filter(|(_, (a, b, c))| a == b && b == c)
                    .flat_map(|(left_index, _)| vec![left_index, left_index + 1])
                    .collect();
                double_indexes.difference(&indexes_in_triples).count() > 0
            })
            .count()
    }
}
