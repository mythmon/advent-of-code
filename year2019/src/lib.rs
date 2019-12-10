use advent_lib::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod intcode;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        day01::get_puzzles(),
        day02::get_puzzles(),
        day03::get_puzzles(),
        day04::get_puzzles(),
        day05::get_puzzles(),
        day06::get_puzzles(),
        day07::get_puzzles(),
        day08::get_puzzles(),
        day09::get_puzzles(),
    ]
    .into_iter()
    .flatten()
    .collect()
}