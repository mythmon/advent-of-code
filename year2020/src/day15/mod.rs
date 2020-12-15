use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
};
use std::{collections::HashMap, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(Part1),
        Box::new(Part2),
    ]
}

struct MemoryGame {
    next_index: usize,
    starting_numbers: Vec<u32>,
    last_seen: HashMap<u32, (Option<usize>, Option<usize>)>,
    last_number: Option<u32>,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<u32>) -> Result<Self, String> {
        if starting_numbers.is_empty() {
            return Err("Starting numbers can't be empty".into());
        }
        let last_seen = HashMap::new();
        Ok(Self {
            next_index: 0,
            starting_numbers,
            last_seen,
            last_number: None,
        })
    }
}

impl Iterator for MemoryGame {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.starting_numbers.get(self.next_index) {
            self.last_number = Some(*v);
        } else {
            self.last_number = Some(match self.last_seen.get(&self.last_number.unwrap()) {
                None => 0,
                Some((None, None)) => 0,
                Some((Some(_), None)) => 0,
                Some((Some(t0), Some(t1))) => (t1 - t0) as u32,
                Some((None, Some(_))) => unreachable!(),
            });
        }
        self.next_index += 1;
        let next_index = self.next_index;
        self.last_seen.entry(self.last_number.unwrap())
            .and_modify(|recent| *recent = match recent.clone() {
                (Some(last), None) => (Some(last), Some(next_index)),
                (Some(_), Some(last)) => (Some(last), Some(next_index)),
                (None, _) => unreachable!(),
            })
            .or_insert((Some(self.next_index), None));
        self.last_number
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<u32>;
    type Output = u32;

    fn name(&self) -> String {
        "2020-D15-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", vec![1, 3, 2], 1)
            .case("Example 2", vec![2, 1, 3], 10)
            .case("Example 3", vec![1, 2, 3], 27)
            .case("Example 4", vec![2, 3, 1], 78)
            .case("Example 5", vec![3, 2, 1], 438)
            .case("Example 6", vec![3, 1, 2], 1836)
            .case("Solution", vec![2, 1, 10, 11, 0, 6], 232)
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        // 2019 because `.nth()` is 0-indexed
        MemoryGame::new(input)?.nth(2019).ok_or("No answer".into())
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<u32>;
    type Output = u32;

    fn name(&self) -> String {
        "2020-D15-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 0", vec![0, 3, 6], 175594)
            .case("Example 1", vec![1, 3, 2], 2578)
            .case("Example 2", vec![2, 1, 3], 3544142)
            .case("Example 3", vec![1, 2, 3], 261214)
            .case("Example 4", vec![2, 3, 1], 6895259)
            .case("Example 5", vec![3, 2, 1], 18)
            .case("Example 6", vec![3, 1, 2], 362)
            .case("Solution", vec![2, 1, 10, 11, 0, 6], 18929178)
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        MemoryGame::new(input)?.nth(29999999).ok_or("No answer".into())
    }
}
