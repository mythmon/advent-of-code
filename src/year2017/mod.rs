use crate::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(day01::part1::Part1),
        Box::new(day01::part2::Part2),
        Box::new(day02::part1::Part1),
        Box::new(day02::part2::Part2),
        Box::new(day03::part1::Part1),
        Box::new(day03::part2::Part2),
        Box::new(day04::part1::Part1),
        Box::new(day04::part2::Part2),
        Box::new(day05::part1::Part1),
        Box::new(day05::part2::Part2),
        Box::new(day06::part1::Part1),
        Box::new(day06::part2::Part2),
        Box::new(day07::part1::Part1),
        Box::new(day07::part2::Part2),
        Box::new(day08::part1::Part1),
        Box::new(day08::part2::Part2),
        Box::new(day09::part1::Part1),
        Box::new(day09::part2::Part2),
        Box::new(day10::part1::Part1),
        Box::new(day10::part2::Part2),
        Box::new(day11::part1::Part1),
        Box::new(day11::part2::Part2),
        Box::new(day12::part1::Part1),
        Box::new(day12::part2::Part2),
        Box::new(day13::part1::Part1),
        Box::new(day13::part2::Part2),
        Box::new(day14::part1::Part1),
        Box::new(day14::part2::Part2),
        Box::new(day15::part1::Part1),
        Box::new(day15::part2::Part2),
        Box::new(day16::part1::Part1),
        Box::new(day16::part2::Part2),
        Box::new(day17::part1::Part1),
        Box::new(day17::part2::Part2),
        Box::new(day18::part1::Part1),
        Box::new(day18::part2::Part2),
        Box::new(day19::part1::Part1),
        Box::new(day19::part2::Part2),
        Box::new(day20::part1::Part1),
        Box::new(day20::part2::Part2),
        Box::new(day21::part1::Part1),
        Box::new(day21::part2::Part2),
        Box::new(day22::part1::Part1),
        Box::new(day22::part2::Part2),
        Box::new(day23::part1::Part1),
        Box::new(day23::part2::Part2),
        Box::new(day24::part1::Part1),
        Box::new(day24::part2::Part2),
        Box::new(day25::Day25),
    ]
}
