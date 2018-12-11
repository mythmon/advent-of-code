use crate::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day01::Day01Part1),
        Box::new(day01::Day01Part2),
        Box::new(day02::Day02Part1),
        Box::new(day02::Day02Part2),
        Box::new(day03::Day03Part1),
        Box::new(day03::Day03Part2),
        Box::new(day04::Day04Part1),
        Box::new(day04::Day04Part2),
        Box::new(day05::Day05Part1),
        Box::new(day05::Day05Part2),
        Box::new(day06::Day06Part1),
        Box::new(day06::Day06Part2),
        Box::new(day07::Day07Part1),
        Box::new(day07::Day07Part2),
        Box::new(day08::Day08Part1),
        Box::new(day08::Day08Part2),
    ]
}
