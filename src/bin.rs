use colored::Colorize;

use advent::{
    cases::Puzzle,
    day01::part1::Day01Part1,
    day01::part2::Day01Part2,
    day02::part1::Day02Part1,
    day02::part2::Day02Part2,
    day03::part1::Day03Part1,
    day03::part2::Day03Part2,
    day04::part1::Day04Part1,
    day04::part2::Day04Part2,
    day05::part1::Day05Part1,
    day05::part2::Day05Part2,
    day06::part1::Day06Part1,
    day06::part2::Day06Part2,
    day07::part1::Day07Part1,
    day07::part2::Day07Part2,
    day08::part1::Day08Part1,
    day08::part2::Day08Part2,
    day09::part1::Day09Part1,
    day09::part2::Day09Part2,
    day10::part1::Day10Part1,
    day10::part2::Day10Part2,
    day11::part1::Day11Part1,
    day11::part2::Day11Part2,
    day12::part1::Day12Part1,
    day12::part2::Day12Part2,
    day13::part1::Day13Part1,
    day13::part2::Day13Part2,
    day14::part1::Day14Part1,
    day14::part2::Day14Part2,
    day15::part1::Day15Part1,
    day15::part2::Day15Part2,
    day16::part1::Day16Part1,
    day16::part2::Day16Part2,
    day17::part1::Day17Part1,
    day17::part2::Day17Part2,
    day18::part1::Day18Part1,
    day18::part2::Day18Part2,
    day19::part1::Day19Part1,
    day19::part2::Day19Part2,
    day20::part1::Day20Part1,
    day20::part2::Day20Part2,
    day21::part1::Day21Part1,
    day21::part2::Day21Part2,
    day22::part1::Day22Part1,
    day22::part2::Day22Part2,
    day23::part1::Day23Part1,
    day23::part2::Day23Part2,
    day24::part1::Day24Part1,
    day24::part2::Day24Part2,
    day25::Day25,
};

fn main() {
    let parts: Vec<Box<dyn Puzzle>> = vec![
        Box::new(Day01Part1 {}),
        Box::new(Day01Part2 {}),
        Box::new(Day02Part1 {}),
        Box::new(Day02Part2 {}),
        Box::new(Day03Part1 {}),
        Box::new(Day03Part2 {}),
        Box::new(Day04Part1 {}),
        Box::new(Day04Part2 {}),
        Box::new(Day05Part1 {}),
        Box::new(Day05Part2 {}),
        Box::new(Day06Part1 {}),
        Box::new(Day06Part2 {}),
        Box::new(Day07Part1 {}),
        Box::new(Day07Part2 {}),
        Box::new(Day08Part1 {}),
        Box::new(Day08Part2 {}),
        Box::new(Day09Part1 {}),
        Box::new(Day09Part2 {}),
        Box::new(Day10Part1 {}),
        Box::new(Day10Part2 {}),
        Box::new(Day11Part1 {}),
        Box::new(Day11Part2 {}),
        Box::new(Day12Part1 {}),
        Box::new(Day12Part2 {}),
        Box::new(Day13Part1 {}),
        Box::new(Day13Part2 {}),
        Box::new(Day14Part1 {}),
        Box::new(Day14Part2 {}),
        Box::new(Day15Part1 {}),
        Box::new(Day15Part2 {}),
        Box::new(Day16Part1 {}),
        Box::new(Day16Part2 {}),
        Box::new(Day17Part1 {}),
        Box::new(Day17Part2 {}),
        Box::new(Day18Part1 {}),
        Box::new(Day18Part2 {}),
        Box::new(Day19Part1 {}),
        Box::new(Day19Part2 {}),
        Box::new(Day20Part1 {}),
        Box::new(Day20Part2 {}),
        Box::new(Day21Part1 {}),
        Box::new(Day21Part2 {}),
        Box::new(Day22Part1 {}),
        Box::new(Day22Part2 {}),
        Box::new(Day23Part1 {}),
        Box::new(Day23Part2 {}),
        Box::new(Day24Part1 {}),
        Box::new(Day24Part2 {}),
        Box::new(Day25 {}),
    ];

    for part in parts {
        print!("{} ", part.name());
        let mut failures = vec![];
        for case in part.cases() {
            if let Ok(_) = case.run() {
                print!("{}", "✔".green());
            } else {
                print!("{}", "✗".red());
                failures.push(case);
            }
        }

        println!();

        if failures.len() > 0 {
            for failing_case in failures {
                println!("  {:<30} FAIL", failing_case.name());
            }
        }
    }
}
