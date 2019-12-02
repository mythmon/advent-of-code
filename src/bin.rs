#![deny(clippy::all)]

use clap::{crate_version, App, AppSettings, Arg, SubCommand};
use colored::Colorize;
use std::{fs, path::PathBuf};

use advent_lib::cases::{Puzzle, PuzzleResult};

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
                .about("Adds a puzzle, templating the code and fetching the input")
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
    let mut puzzles = year2015::get_puzzles();
    puzzles.extend(year2017::get_puzzles());
    puzzles.extend(year2018::get_puzzles());
    puzzles.extend(year2019::get_puzzles());
    puzzles
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

    for puzzle in puzzles {
        let results: Vec<_> = puzzle
            .cases()
            .into_iter()
            .map(|case| {
                let result = case.run();
                (case, result)
            })
            .collect();

        print!("{:<12}", puzzle.name());
        if opts.verbose {
            println!();
            for (case, result) in results {
                print!(
                    "    {} ",
                    match result {
                        PuzzleResult::Match => "PASS".green(),
                        PuzzleResult::Unknown { .. } => "UNKO".yellow(),
                        PuzzleResult::Fail { .. } => "FAIL".red(),
                    }
                );
                print!("{:<10}", case.name());
                match result {
                    PuzzleResult::Match => (),
                    PuzzleResult::Unknown { description } => print!(" -> {}", description),
                    PuzzleResult::Fail { description } => print!(" -> {}", description),
                }
                println!();
            }
        } else {
            for (_, result) in &results {
                match result {
                    PuzzleResult::Match => print!("{}", "✔".green()),
                    PuzzleResult::Unknown { .. } => print!("{}", "?".yellow()),
                    PuzzleResult::Fail { .. } => print!("{}", "✗".red()),
                }
            }
            println!();
            for (case, result) in results {
                match result {
                    PuzzleResult::Unknown { description } => println!(
                        "   {} {:<10} -> {}",
                        "UNKO".yellow(),
                        case.name(),
                        description
                    ),
                    PuzzleResult::Fail { description } => {
                        println!("   {} {:<10} -> {}", "FAIL".red(), case.name(), description)
                    }
                    _ => (),
                }
            }
        }
    }
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

    let day_padded = format!("{:0>2}", opts.day.to_string());

    let puzzle_path = PathBuf::from(format!("./src/year{}/day{}", opts.year, day_padded));
    fs::create_dir_all(&puzzle_path)?;

    let mut mod_path = puzzle_path.clone();
    mod_path.push("mod.rs");
    if !mod_path.exists() {
        let mod_template = String::from_utf8(fs::read("./template/mod.rs.tmpl")?)?
            .replace("{{YEAR}}", &opts.year.to_string())
            .replace("{{DAY_PADDED}}", &day_padded);
        fs::write(mod_path, mod_template)?;
    }

    let mut input_path = puzzle_path;
    input_path.push("input");
    if !input_path.exists() {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            opts.year, opts.day,
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
