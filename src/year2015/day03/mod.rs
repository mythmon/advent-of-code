use crate::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::{collections::HashSet, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2015-D03-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", ">", 2)
            .case("Example 2", "^>v<", 4)
            .case("Example 3", "^v^v^v^v^v", 2)
            .case("Solution", include_str!("input"), 2081)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut visited: HashSet<(i32, i32)> = input
            .chars()
            .filter_map(|c| match c {
                '^' => Some((0, -1)),
                '>' => Some((1, 0)),
                'v' => Some((0, 1)),
                '<' => Some((-1, 0)),
                _ => None,
            })
            .scan((0, 0), |state, delta| -> Option<(i32, i32)> {
                state.0 += delta.0;
                state.1 += delta.1;
                Some(*state)
            })
            .collect::<HashSet<_>>();

        // we started at (0, 0), so add that as well
        visited.insert((0, 0));

        visited.len()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2015-D03-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "^v", 3)
            .case("Example 2", "^>v<", 3)
            .case("Example 3", "^v^v^v^v^v", 11)
            .case("Solution", include_str!("input"), 2341)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let instructions: Vec<_> = input
            .chars()
            .filter_map(|c| match c {
                '^' => Some((0, -1)),
                '>' => Some((1, 0)),
                'v' => Some((0, 1)),
                '<' => Some((-1, 0)),
                _ => None,
            })
            .collect();

        let mut visited: HashSet<(i32, i32)> = instructions
            .chunks_exact(2)
            .scan(((0, 0), (0, 0)), |(santa_pos, robot_pos), deltas| {
                let santa_delta = deltas[0];
                let robot_delta = deltas[1];
                santa_pos.0 += santa_delta.0;
                santa_pos.1 += santa_delta.1;
                robot_pos.0 += robot_delta.0;
                robot_pos.1 += robot_delta.1;
                Some(vec![*santa_pos, *robot_pos])
            })
            .flatten()
            .collect::<HashSet<_>>();

        // we started at (0, 0), so add that as well
        visited.insert((0, 0));

        visited.len()
    }
}
