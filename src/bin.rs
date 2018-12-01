#![deny(clippy::all)]

use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use rayon::prelude::*;
use std::{collections::BTreeMap, fs, path::PathBuf};

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
        .subcommand(
            SubCommand::with_name("add-puzzle")
                .about("Add's a puzzle, templating the code and fetching the input")
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::with_name("day")
                        .takes_value(true)
                        .help("The day of the puzzle to add (1 through 25)")
                        .required(true),
                )
                .arg(
                    Arg::with_name("year")
                        .short("y")
                        .long("year")
                        .default_value("2018")
                        .takes_value(true)
                        .help("The year of the puzzle to add defaults to 2018")
                        .required(false),
                )
                .arg(
                    Arg::with_name("advent_cookie")
                        .long("advent-cookie")
                        .env("ADVENT_COOKIE")
                        .help("Session cookie from adventofcode.com")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(opts)) => run(opts),
        ("add-puzzle", Some(opts)) => add_puzzle(opts).unwrap(),
        _ => {
            println!("Unknown command!");
            std::process::exit(1);
        }
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

struct AddDayOptions {
    day: u8,
    year: u16,
    advent_cookie: String,
}

impl<'a> From<&clap::ArgMatches<'a>> for AddDayOptions {
    fn from(matches: &clap::ArgMatches) -> Self {
        Self {
            day: matches.value_of("day").unwrap().parse().unwrap(),
            year: matches.value_of("year").unwrap().parse().unwrap(),
            advent_cookie: matches.value_of("advent_cookie").unwrap().to_owned(),
        }
    }
}

fn add_puzzle<O>(opts: O) -> Result<(), Box<dyn std::error::Error>>
where
    O: Into<AddDayOptions>,
{
    let opts = opts.into();

    let puzzle_path = PathBuf::from(format!("./src/year{}/day{}", opts.year, opts.day));
    fs::create_dir_all(&puzzle_path)?;

    let mut mod_path = puzzle_path.clone();
    mod_path.push("mod.rs");
    if !mod_path.exists() {
        let mod_template = String::from_utf8(fs::read("./template/mod.rs.tmpl")?)?
            .replace("{{YEAR}}", &opts.year.to_string())
            .replace("{{DAY_PADDED}}", &format!("{:0>2}", opts.day.to_string()));
        fs::write(mod_path, mod_template)?;
    }

    let mut input_path = puzzle_path.clone();
    input_path.push("input");
    if !input_path.exists() {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            opts.year, opts.day
        );
        let client = reqwest::Client::new();
        let input = client
            .get(&url)
            .header(
                reqwest::header::COOKIE,
                format!("session={}", opts.advent_cookie),
            )
            .send()?
            .error_for_status()?
            .text()?;
        fs::write(input_path, input)?;
    }

    Ok(())
}
