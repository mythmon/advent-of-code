use colored::Colorize;

use advent::{
    day01::{part1::Day01Part1, part2::Day01Part2},
    PuzzlePart,
};

fn main() {
    let parts: Vec<Box<dyn PuzzlePart<Input = _, Output = _>>> =
        vec![Box::new(Day01Part1 {}), Box::new(Day01Part2 {})];

    for part in parts {
        print!("{} ", part.name());
        let mut failures = vec![];
        for case in part.cases() {
            let actual_output = part.run(&case);
            if actual_output == case.output {
                print!("{}", "✔".green());
            } else {
                print!("{}", "✗".red());
                failures.push(case);
            }
        }

        for failure in failures {
            println!("  {:>30} FAIL", failure.name);
        }

        println!("");
    }
}
