use crate::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
// mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day01::Part1),
        Box::new(day01::Part2),
        Box::new(day02::Part1),
        Box::new(day02::Part2),
        Box::new(day03::Part1),
        Box::new(day03::Part2),
        Box::new(day04::Part1),
        Box::new(day04::Part2),
        Box::new(day05::Part1),
        Box::new(day05::Part2),
        Box::new(day06::Part1),
        Box::new(day06::Part2),
        Box::new(day07::Part1),
        Box::new(day07::Part2),
        // TODO accidentally deleted day 8, recreate it
        // Box::new(day08::Part1),
        // Box::new(day08::Part2),
        Box::new(day09::Part1),
        Box::new(day09::Part2),
        Box::new(day10::Part1),
        Box::new(day10::Part2),
        Box::new(day11::Part1),
        Box::new(day11::Part2),
        Box::new(day12::Part1),
        Box::new(day12::Part2),
    ]
}
