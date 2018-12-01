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
        Box::new(day01::part1::Day01Part1),
        Box::new(day01::part2::Day01Part2),
        Box::new(day02::part1::Day02Part1),
        Box::new(day02::part2::Day02Part2),
        Box::new(day03::part1::Day03Part1),
        Box::new(day03::part2::Day03Part2),
        Box::new(day04::part1::Day04Part1),
        Box::new(day04::part2::Day04Part2),
        Box::new(day05::part1::Day05Part1),
        Box::new(day05::part2::Day05Part2),
        Box::new(day06::part1::Day06Part1),
        Box::new(day06::part2::Day06Part2),
        Box::new(day07::part1::Day07Part1),
        Box::new(day07::part2::Day07Part2),
        Box::new(day08::part1::Day08Part1),
        Box::new(day08::part2::Day08Part2),
        Box::new(day09::part1::Day09Part1),
        Box::new(day09::part2::Day09Part2),
        Box::new(day10::part1::Day10Part1),
        Box::new(day10::part2::Day10Part2),
        Box::new(day11::part1::Day11Part1),
        Box::new(day11::part2::Day11Part2),
        Box::new(day12::part1::Day12Part1),
        Box::new(day12::part2::Day12Part2),
        Box::new(day13::part1::Day13Part1),
        Box::new(day13::part2::Day13Part2),
        Box::new(day14::part1::Day14Part1),
        Box::new(day14::part2::Day14Part2),
        Box::new(day15::part1::Day15Part1),
        Box::new(day15::part2::Day15Part2),
        Box::new(day16::part1::Day16Part1),
        Box::new(day16::part2::Day16Part2),
        Box::new(day17::part1::Day17Part1),
        Box::new(day17::part2::Day17Part2),
        Box::new(day18::part1::Day18Part1),
        Box::new(day18::part2::Day18Part2),
        Box::new(day19::part1::Day19Part1),
        Box::new(day19::part2::Day19Part2),
        Box::new(day20::part1::Day20Part1),
        Box::new(day20::part2::Day20Part2),
        Box::new(day21::part1::Day21Part1),
        Box::new(day21::part2::Day21Part2),
        Box::new(day22::part1::Day22Part1),
        Box::new(day22::part2::Day22Part2),
        Box::new(day23::part1::Day23Part1),
        Box::new(day23::part2::Day23Part2),
        Box::new(day24::part1::Day24Part1),
        Box::new(day24::part2::Day24Part2),
        Box::new(day25::Day25),
    ]
}
