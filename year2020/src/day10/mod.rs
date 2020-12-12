use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{collections::HashMap, error::Error, iter::Iterator, num::ParseIntError};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<i32>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D10-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .case("Example 1", vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4], 35)
            .case(
                "Example 2",
                vec![
                    28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32,
                    25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
                ],
                220,
            )
            .transformed_case("Solution", include_str!("input"), 2_272)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        if input.is_empty() {
            return Ok(0);
        }

        let mut with_extra = Vec::with_capacity(input.len() + 2);
        with_extra.push(0);
        with_extra.extend(input.into_iter());
        with_extra.sort_unstable();
        with_extra.push(with_extra[with_extra.len() - 1] + 3);

        let mut diffs = HashMap::new();

        for i in 0..(with_extra.len() - 1) {
            let diff = with_extra[i + 1] - with_extra[i];
            *diffs.entry(diff).or_insert(0) += 1;
        }

        Ok(diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0))
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<i32>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D10-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .case("Example 1", vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4], 8)
            .case(
                "Example 2",
                vec![
                    28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32,
                    25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
                ],
                19_208,
            )
            .transformed_case("Solution", include_str!("input"), 84_627_647_627_264)?
            .collect())
    }

    fn try_run_puzzle(mut converters: Self::Input) -> Result<Self::Output, Self::Error> {
        if converters.is_empty() {
            return Ok(0);
        }

        converters.sort_unstable();
        let source = 0;
        let target = converters[converters.len() - 1] + 3;

        let mut path_counts_by_index = HashMap::new();

        for i in (0..converters.len()).rev() {
            if path_counts_by_index.contains_key(&i) {
                return Err("Bug! Found duplicate index in counts".into());
            }
            let mut count = 0_u64;
            if target - converters[i] <= 3 {
                count += 1;
            }
            for j in (i + 1)..converters.len() {
                if converters[j] - converters[i] <= 3 {
                    count += path_counts_by_index[&j];
                } else {
                    break;
                }
            }
            path_counts_by_index.insert(i, count);
        }

        let mut source_count = 0;
        for i in 0..converters.len() {
            if converters[i] - source <= 3 {
                source_count += path_counts_by_index[&i];
            } else {
                break;
            }
        }

        Ok(source_count)
    }
}

fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    input
        .trimmed_lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err: ParseIntError| err.into())
}
