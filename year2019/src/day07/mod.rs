use crate::intcode::{IntcodeComputer, PauseReason};
use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D07-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case(
                "Example 1",
                vec![
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                ],
                43_210,
            )
            .case(
                "Example 2",
                vec![
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0,
                ],
                54_321,
            )
            .case(
                "Example 3",
                vec![
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
                ],
                65_210,
            )
            .transformed_case("Solution", include_str!("input"), 880_726)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        all_permutations(vec![0, 1, 2, 3, 4])
            .into_iter()
            .map(|phase_sequence| {
                let mut next_input = 0;
                for phase in phase_sequence {
                    let mut computer = IntcodeComputer::build(program.clone())
                        .with_input(vec![phase, next_input])
                        .done();
                    computer.run_to_end();
                    next_input = computer.output[0];
                }
                next_input
            })
            .max()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<isize>;
    type Output = isize;

    fn name(&self) -> String {
        "2019-D07-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .case(
                "Example 1",
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ],
                139_629_729,
            )
            .case(
                "Example 2",
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
                ],
                18_216,
            )
            .transformed_case("Solution", include_str!("input"), 4_931_744)
            .collect()
    }

    fn run_puzzle(program: Self::Input) -> Self::Output {
        let phases: Vec<isize> = vec![5, 6, 7, 8, 9];
        all_permutations(phases)
            .into_iter()
            .map(|phase_sequence| {
                let mut amplifiers: Vec<_> = phase_sequence
                    .iter()
                    .map(|p| {
                        IntcodeComputer::build(program.clone())
                            // .verbose()
                            .with_input(vec![*p])
                            .done()
                    })
                    .collect();

                let mut next_input = 0;

                // feed the signal through the amplifiers repeatedly, until they
                // stop producing output (when one halts)
                for amplifier_id in (0..5).cycle() {
                    let amp = &mut amplifiers[amplifier_id];
                    amp.add_input(next_input);
                    match amp.run_until_io() {
                        PauseReason::Output(signal) => {
                            next_input = signal;
                        }
                        PauseReason::Input => {
                            panic!("Mismatched input");
                        }
                        PauseReason::Halt => break,
                    }
                }

                next_input
            })
            .max()
            .unwrap()
    }
}

fn all_permutations<T>(mut items: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    // Heap's algorithm
    let mut c: Vec<usize> = Vec::with_capacity(items.len());
    c.resize_with(items.len(), || 0); // fill with 0s
    let mut rv = Vec::with_capacity(items.len().factorial());
    rv.push(items.clone());

    let mut i = 0;
    while i < items.len() {
        if c[i] < i {
            if i % 2 == 0 {
                items.swap(0, i);
            } else {
                items.swap(c[i], i);
            }
            rv.push(items.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    rv
}

trait Factorial {
    fn factorial(&self) -> Self;
}

impl Factorial for usize {
    fn factorial(&self) -> Self {
        (2..=*self).product()
    }
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_permutations() {
        let v = vec![1, 2, 3];
        let permutations = all_permutations(v);
        let expected = vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 2, 1],
        ];
        assert_eq!(permutations, expected);
    }
}
