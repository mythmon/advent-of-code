#![deny(clippy::all)]

use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use rayon::prelude::*;
use std::collections::BTreeMap;

use advent::{cases::Puzzle, year2017};

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
    year2017::get_puzzles()
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
