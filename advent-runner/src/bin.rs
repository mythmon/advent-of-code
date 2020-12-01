#![deny(clippy::all)]

use advent_lib::cases::{Puzzle, PuzzleResultStatus};
use colored::Colorize;
use num_format::{Locale, ToFormattedString};
use reqwest::StatusCode;
use std::{fmt, fs, path::PathBuf, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Verbose mode, can be repeated (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Runs puzzles
    Run {
        /// Only run tests who's name contains this string
        #[structopt()]
        filter: Vec<String>,

        /// Verbose mode, can be repeated (-v, -vv, -vvv, etc.)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: u8,
    },

    /// Lists puzzle that could run
    List {
        /// Only run tests who's name contains this string
        #[structopt()]
        filter: Vec<String>,

        /// Verbose mode, can be repeated (-v, -vv, -vvv, etc.)
        #[structopt(short, long, parse(from_occurrences))]
        verbose: u8,
    },

    /// Adds a puzzle, templating the code and fetching the input
    AddDay {
        /// The day of the puzzle to add (1 through 25)
        #[structopt(short, long)]
        day: u8,

        /// The year of the puzzle to add defaults to 2019
        #[structopt(short, long, default_value = "2019")]
        year: u16,

        /// Session cookie from adventofcode.com
        #[structopt(short, long, env = "ADVENT_COOKIE", hide_env_values = true)]
        advent_cookie: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::Run { .. } => run(opt),
        Command::List { .. } => list(opt),
        Command::AddDay { .. } => add_puzzle(opt)?,
    }

    Ok(())
}

fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    let mut puzzles = vec![];

    #[cfg(feature = "year2015")]
    puzzles.extend(year2015::get_puzzles());
    #[cfg(feature = "year2017")]
    puzzles.extend(year2017::get_puzzles());
    #[cfg(feature = "year2018")]
    puzzles.extend(year2018::get_puzzles());
    #[cfg(feature = "year2019")]
    puzzles.extend(year2019::get_puzzles());

    puzzles
}

struct RunOptions {
    filter: Vec<String>,
    verbose: bool,
}

impl<'a> From<Opt> for RunOptions {
    fn from(opt: Opt) -> Self {
        let Opt {
            verbose: top_verbose,
            cmd,
            ..
        } = opt;
        if let Command::Run {
            filter,
            verbose: cmd_verbose,
        } = cmd
        {
            Self {
                filter,
                verbose: cmd_verbose + top_verbose > 0,
            }
        } else {
            panic!("Incorrect subcommand, expected run");
        }
    }
}

fn run<O: Into<RunOptions>>(opts: O) {
    let opts = opts.into();

    let filter_parts: Vec<String> = opts
        .filter
        .iter()
        .flat_map(|f| f.split(' '))
        .map(|p| p.to_string().to_lowercase())
        .collect();

    for puzzle in get_puzzles() {
        let results: Vec<_> = puzzle
            .cases()
            .into_iter()
            .filter(|case| {
                if !filter_parts.is_empty() {
                    let haystack = format!("{} {}", puzzle.name(), case.name()).to_lowercase();
                    filter_parts.iter().all(|needle| haystack.contains(needle))
                } else {
                    true
                }
            })
            .map(|case| {
                let result = case.run();
                (case, result)
            })
            .collect();

        if results.is_empty() {
            continue;
        }

        print!("{:<12}", puzzle.name());
        if opts.verbose {
            println!();
            for (case, result) in results {
                print!(
                    "    {} ",
                    match result.status {
                        PuzzleResultStatus::Match => "PASS".green(),
                        PuzzleResultStatus::Unknown => "UNKO".yellow(),
                        PuzzleResultStatus::Fail => "FAIL".red(),
                    }
                );
                print!("{:<10} ", case.name());
                match result.status {
                    PuzzleResultStatus::Match => (),
                    PuzzleResultStatus::Unknown => print!(" -> {}", result.description),
                    PuzzleResultStatus::Fail => print!(" -> {}", result.description),
                }
                print!("{}", format_sum_duration(vec![result.duration]));
                println!();
            }
        } else {
            for (_, result) in &results {
                match result.status {
                    PuzzleResultStatus::Match => print!("{}", "✔".green()),
                    PuzzleResultStatus::Unknown => print!("{}", "?".yellow()),
                    PuzzleResultStatus::Fail => print!("{}", "✗".red()),
                }
            }

            let spacer = (results.len()..10).map(|_| " ").collect::<String>();
            println!(
                "{}{}",
                spacer,
                format_sum_duration(results.iter().map(|(_, res)| res.duration).collect())
            );

            for (case, result) in results {
                match result.status {
                    PuzzleResultStatus::Unknown => println!(
                        "   {} {:<10} -> {}",
                        "UNKO".yellow(),
                        case.name(),
                        result.description
                    ),
                    PuzzleResultStatus::Fail => println!(
                        "   {} {:<10} -> {}",
                        "FAIL".red(),
                        case.name(),
                        result.description
                    ),
                    _ => (),
                }
            }
        }
    }
}

fn format_sum_duration(ds: Vec<Duration>) -> impl fmt::Display {
    let sum: u128 = ds.iter().map(Duration::as_micros).sum();
    let s = sum.to_formatted_string(&Locale::en);
    let s = format!("{:>10} us ", s);
    match sum {
        0 => s.bright_black(),
        d if d < 1_000 => s.bright_black(),
        d if d < 10_000 => s.blue(),
        d if d < 100_000 => s.yellow(),
        d if d < 1_000_000 => s.red(),
        _ => s.black().on_red(),
    }
}

#[derive(Debug)]
struct AddDayOptions {
    day: u8,
    year: u16,
    advent_cookie: String,
}

impl<'a> From<Opt> for AddDayOptions {
    fn from(opt: Opt) -> Self {
        let Opt { cmd, .. } = opt;
        if let Command::AddDay {
            day,
            year,
            advent_cookie,
        } = cmd
        {
            Self {
                day,
                year,
                advent_cookie,
            }
        } else {
            panic!("Incorrect subcommand, expected run");
        }
    }
}

fn add_puzzle<O>(opts: O) -> Result<(), Box<dyn std::error::Error>>
where
    O: Into<AddDayOptions>,
{
    let opts = opts.into();

    let day_padded = format!("{:0>2}", opts.day.to_string());

    let puzzle_path = PathBuf::from(format!("./year{}/src/day{}", opts.year, day_padded));
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
        let mut res = client
            .get(&url)
            .header(
                reqwest::header::COOKIE,
                format!("session={}", opts.advent_cookie),
            )
            .send()?;

        if res.status() == StatusCode::NOT_FOUND {
            println!("No input for this puzzle");
        } else {
            let body = res.text()?;
            if !res.status().is_success() {
                println!("Error: {}", body);
                res.error_for_status()?;
            }
            fs::write(input_path, body)?;
        }
    }

    Ok(())
}

struct ListOptions {
    filter: Vec<String>,
    verbose: bool,
}

impl<'a> From<Opt> for ListOptions {
    fn from(opt: Opt) -> Self {
        let Opt {
            verbose: top_verbose,
            cmd,
            ..
        } = opt;
        if let Command::List {
            filter,
            verbose: cmd_verbose,
        } = cmd
        {
            Self {
                filter,
                verbose: cmd_verbose + top_verbose > 0,
            }
        } else {
            panic!("Incorrect subcommand, expected list");
        }
    }
}

fn list<O: Into<ListOptions>>(opts: O) {
    let opts = opts.into();

    let filter_parts: Vec<String> = opts
        .filter
        .iter()
        .flat_map(|f| f.split(' '))
        .map(|p| p.to_string().to_lowercase())
        .collect();

    for puzzle in get_puzzles() {
        let results: Vec<_> = puzzle
            .cases()
            .into_iter()
            .filter(|case| {
                if !filter_parts.is_empty() {
                    let haystack = format!("{} {}", puzzle.name(), case.name()).to_lowercase();
                    filter_parts.iter().all(|needle| haystack.contains(needle))
                } else {
                    true
                }
            })
            .collect();

        if results.is_empty() {
            continue;
        }

        println!("{:<12}", puzzle.name());

        if opts.verbose {
            for case in results {
                println!("  {:<10} ", case.name());
            }
        }
    }
}
