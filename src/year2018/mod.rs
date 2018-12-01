use crate::cases::Puzzle;

mod day01;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(day01::Day01Part1), Box::new(day01::Day01Part2)]
}
