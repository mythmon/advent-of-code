#![deny(clippy::all)]

use advent_lib::cases::{Puzzle, PuzzleResultStatus};
use colored::Colorize;
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
        filter: Option<String>,

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
        Command::AddDay { .. } => add_puzzle(opt)?,
    }

    Ok(())
}

struct RunOptions {
    filter: Option<String>,
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
                filter: filter,
                verbose: cmd_verbose + top_verbose > 0,
            }
        } else {
            panic!("Incorrect subcommand, expected run");
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
    let sum: u128 = ds.iter().map(Duration::as_millis).sum();
    let s = format!("{:>5} ms ", sum);
    match sum {
        0 => s.bright_black(),
        d if d < 100 => s.bright_black(),
        d if d < 200 => s.blue(),
        d if d < 1_000 => s.yellow(),
        d if d < 2_000 => s.red(),
        _ => s.black().on_red(),
    }
}

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
