use crate::cases::Puzzle;

mod day01;
mod day02;
mod day03;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day01::Day01Part1),
        Box::new(day01::Day01Part2),
        Box::new(day02::Day02Part1),
        Box::new(day02::Day02Part2),
        Box::new(day03::Day03Part1),
        Box::new(day03::Day03Part2),
    ]
}
