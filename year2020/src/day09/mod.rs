use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::{Bounds, StringAdventExt},
};
use itertools::Itertools;
use std::{error::Error, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = (usize, Vec<u64>);
    type Output = u64;

    fn name(&self) -> String {
        "2020-D09-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .case(
                "Example",
                (
                    5,
                    vec![
                        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299,
                        277, 309, 576,
                    ],
                ),
                127,
            )
            .transformed_case("Solution", include_str!("input"), 217_430_975)?
            .collect())
    }

    fn try_run_puzzle((window_size, input): Self::Input) -> Result<Self::Output, Self::Error> {
        let mut log = Vec::with_capacity(input.len());
        let input_iter = input.into_iter();
        log.extend(input_iter.clone().take(window_size));

        for n in input_iter.skip(window_size) {
            let window = &log[log.len() - window_size..log.len()];
            if !window
                .iter()
                .combinations(2)
                .any(|combo| combo.into_iter().sum::<u64>() == n)
            {
                return Ok(n);
            }
            log.push(n);
        }

        Err("No answer".into())
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = (usize, Vec<u64>);
    type Output = u64;

    fn name(&self) -> String {
        "2020-D09-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .case(
                "Example",
                (
                    5,
                    vec![
                        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299,
                        277, 309, 576,
                    ],
                ),
                62,
            )
            .transformed_case("Solution", include_str!("input"), 28_509_180)?
            .collect())
    }

    fn try_run_puzzle((window_size, input): Self::Input) -> Result<Self::Output, Self::Error> {
        let goal = Part1::try_run_puzzle((window_size, input.clone()))?;

        for start in 0..(input.len() - 2) {
            let mut sum = input[start];
            for end in (start + 1)..input.len() {
                sum += input[end];
                match sum.cmp(&goal) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => {
                        return input[start..=end]
                            .iter()
                            .bounds()
                            .ok_or_else(|| "Matched range was empty?".into())
                            .map(|(min, max)| *min + *max)
                    }
                    std::cmp::Ordering::Greater => break,
                }
            }
        }

        Err("No answer found".into())
    }
}

fn parse_input(input: &str) -> Result<(usize, Vec<u64>), Box<dyn Error>> {
    input
        .trimmed_lines()
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()
        .map_err(|err| err.into())
        .map(|vec| (25, vec))
}
