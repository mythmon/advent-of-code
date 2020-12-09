use advent_lib::{cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner}, helpers::{Bounds, StringAdventExt}};
use std::{collections::HashSet, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<String>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D05-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case("Example", vec!["FBFBBFFRLR".to_string()], 357)
            .transformed_case("Solution", include_str!("input"), 974)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.into_iter().map(|s| seat_id(&s)).max().unwrap_or(0)
    }
}

fn seat_id(s: &str) -> i32 {
    s.chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!(format!("Unexpected char {}", c)),
        })
        .fold(0, |acc, bit| (acc << 1) + bit)
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<String>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D05-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 646)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let seen_seats: HashSet<_> = input.into_iter().map(|s| seat_id(&s)).collect();
        let (min_seat, max_seat) = seen_seats.iter().bounds().unwrap();
        let all_seats: HashSet<_> = (*min_seat..=*max_seat).into_iter().collect();
        *all_seats.difference(&seen_seats).next().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<String> {
    input.trimmed_lines().map(|line| line.to_string()).collect()
}
