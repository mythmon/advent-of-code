use crate::{PuzzleCase, PuzzlePart};
use std::iter::Iterator;

pub struct Day01Part2;

impl PuzzlePart for Day01Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> &'static str {
        "2017-D01-P2"
    }

    fn cases(&self) -> Vec<PuzzleCase<Self::Input, Self::Output>> {
        vec![
            PuzzleCase {
                name: "Example 1",
                input: "1212",
                output: 6,
            },
            PuzzleCase {
                name: "Example 2",
                input: "1221",
                output: 0,
            },
            PuzzleCase {
                name: "Example 3",
                input: "123425",
                output: 4,
            },
            PuzzleCase {
                name: "Example 4",
                input: "123123",
                output: 12,
            },
            PuzzleCase {
                name: "Example 5",
                input: "12131415",
                output: 4,
            },
            PuzzleCase {
                name: "Solution",
                input: include_str!("input"),
                output: 950,
            },
        ]
    }

    fn puzzle(&self, input: &Self::Input) -> Self::Output {
        puzzle(input)
    }
}

fn puzzle(input: &str) -> u32 {
    let input: Vec<u32> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let offset_input = input.iter().cycle().skip(input.len() / 2);

    let mut sum: u32 = 0;
    for (&a, &b) in input.iter().zip(offset_input) {
        if a == b {
            sum += a;
        }
    }
    sum
}

#[test]
fn examples() {
    assert_eq!(puzzle("1212"), 6);
    assert_eq!(puzzle("1221"), 0);
    assert_eq!(puzzle("123425"), 4);
    assert_eq!(puzzle("123123"), 12);
    assert_eq!(puzzle("12131415"), 4);
}

#[test]
fn correct_solution() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 950);
}
