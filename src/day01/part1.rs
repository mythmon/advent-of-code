use crate::{PuzzleCase, PuzzlePart};
use std::iter::Iterator;

pub struct Day01Part1;

impl PuzzlePart for Day01Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> &'static str {
        "2017-D01-P1"
    }

    fn cases(&self) -> Vec<PuzzleCase<Self::Input, Self::Output>> {
        vec![
            PuzzleCase {
                name: "Example 1",
                input: "1122",
                output: 3,
            },
            PuzzleCase {
                name: "Example 2",
                input: "1111",
                output: 4,
            },
            PuzzleCase {
                name: "Example 3",
                input: "1234",
                output: 0,
            },
            PuzzleCase {
                name: "Example 4",
                input: "91212129",
                output: 9,
            },
            PuzzleCase {
                name: "Solution",
                input: include_str!("input"),
                output: 1141,
            },
        ]
    }

    fn puzzle(&self, input: &Self::Input) -> Self::Output {
        puzzle(input)
    }
}

fn puzzle(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first = input.first().unwrap().clone();
    input.push(first);

    let mut sum = 0;
    for (&a, &b) in input.iter().zip(&input[1..]) {
        if a == b {
            sum += a;
        }
    }
    sum
}

#[test]
fn examples() {
    assert_eq!(puzzle("1122"), 3);
    assert_eq!(puzzle("1111"), 4);
    assert_eq!(puzzle("1234"), 0);
    assert_eq!(puzzle("91212129"), 9);
}

#[test]
fn correct_solution() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 1141);
}
