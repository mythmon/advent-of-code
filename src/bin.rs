#![deny(clippy::all)]

use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use rayon::prelude::*;
use std::collections::BTreeMap;

use advent::{
    cases::Puzzle,
    day01::{part1::Day01Part1, part2::Day01Part2},
    day02::{part1::Day02Part1, part2::Day02Part2},
    day03::{part1::Day03Part1, part2::Day03Part2},
    day04::{part1::Day04Part1, part2::Day04Part2},
    day05::{part1::Day05Part1, part2::Day05Part2},
    day06::{part1::Day06Part1, part2::Day06Part2},
    day07::{part1::Day07Part1, part2::Day07Part2},
    day08::{part1::Day08Part1, part2::Day08Part2},
    day09::{part1::Day09Part1, part2::Day09Part2},
    day10::{part1::Day10Part1, part2::Day10Part2},
    day11::{part1::Day11Part1, part2::Day11Part2},
    day12::{part1::Day12Part1, part2::Day12Part2},
    day13::{part1::Day13Part1, part2::Day13Part2},
    day14::{part1::Day14Part1, part2::Day14Part2},
    day15::{part1::Day15Part1, part2::Day15Part2},
    day16::{part1::Day16Part1, part2::Day16Part2},
    day17::{part1::Day17Part1, part2::Day17Part2},
    day18::{part1::Day18Part1, part2::Day18Part2},
    day19::{part1::Day19Part1, part2::Day19Part2},
    day20::{part1::Day20Part1, part2::Day20Part2},
    day21::{part1::Day21Part1, part2::Day21Part2},
    day22::{part1::Day22Part1, part2::Day22Part2},
    day23::{part1::Day23Part1, part2::Day23Part2},
    day24::{part1::Day24Part1, part2::Day24Part2},
    day25::Day25,
};

fn main() {
    let matches = App::new("Advent")
        .version(crate_version!())
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::ArgsNegateSubcommands,
            AppSettings::DeriveDisplayOrder,
            AppSettings::InferSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::UnifiedHelpMessage,
        ])
        .arg(
            Arg::with_name("verbose")
                .global(true)
                .multiple(true)
                .short("v")
                .long("verbose")
                .help("Print more verbose output"),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs puzzles")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::with_name("filter")
                        .takes_value(true)
                        .help("Only run tests whose name contains this string")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(opts)) => run(opts),
        _ => unreachable!(),
    }
}

struct RunOptions {
    filter: Option<String>,
    verbose: bool,
}

impl<'a> From<&clap::ArgMatches<'a>> for RunOptions {
    fn from(matches: &clap::ArgMatches) -> Self {
        Self {
            filter: matches.value_of("filter").map(|f| f.to_owned()),
            verbose: matches.is_present("verbose"),
        }
    }
}

fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
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
    ]
}

fn run<O>(opts: O)
where
    O: Into<RunOptions>,
{
    let opts = opts.into();
    let mut puzzles = get_puzzles();

    if let Some(ref filter) = opts.filter {
        let filter = filter.to_lowercase();
        puzzles = puzzles
            .into_iter()
            .filter(|p| p.name().to_lowercase().contains(&filter))
            .collect();
    }

    puzzles
        .par_iter()
        .flat_map(|part| {
            part.cases()
                .into_iter()
                .map(|case| (part, case))
                .collect::<Vec<_>>()
        })
        .map(|(part, case)| {
            let result = case.run();
            (part, case, result)
        })
        .collect::<Vec<_>>()
        .iter()
        .fold(BTreeMap::new(), |mut map, (part, case, result)| {
            map.entry(part.name())
                .or_insert_with(Vec::new)
                .push((case, result));
            map
        })
        .iter()
        .for_each(|(group_name, results)| {
            print!("{} ", group_name);
            if opts.verbose {
                println!();
                for (case, result) in results {
                    if result.is_ok() {
                        print!("    {} ", "PASS".green());
                    } else {
                        print!("    {} ", "FAIL".red());
                    }
                    println!("{}", case.name());
                }
            } else {
                for (_, result) in results {
                    if result.is_ok() {
                        print!("{}", "✔".green());
                    } else {
                        print!("{}", "✗".red());
                    }
                }
                println!();
                for (case, _) in results.iter().filter(|(_, result)| result.is_err()) {
                    println!("  FAIL {}", case.name());
                }
            }
        });
}
