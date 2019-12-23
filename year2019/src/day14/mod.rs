use advent_lib::{
    cases::{/* ExpectedValue, */ GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{cmp::Ordering, collections::HashMap, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D14-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example1", include_str!("example1"), 31)
            .case("Example2", include_str!("example2"), 165)
            .case("Example3", include_str!("example3"), 133_12)
            .case("Example4", include_str!("example4"), 180_697)
            .case("Example5", include_str!("example5"), 2_210_736)
            .case("Solution", include_str!("input"), 907_302)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let input = input.to_owned();
        let rule_map = parse_input(&input);
        make_fuel(1, &rule_map)
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D14-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example3", include_str!("example3"), 82_892_753)
            .case("Example4", include_str!("example4"), 5_586_022)
            .case("Example5", include_str!("example5"), 460_664)
            .case("Solution", include_str!("input"), 1_670_299)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let target_ore = 1_000_000_000_000;
        let input = input.to_owned();
        let rule_map = parse_input(&input);

        let mut upper_bound_fuel: usize = 1_000_000_000_000;
        let mut lower_bound_fuel: usize = 100_000;

        while lower_bound_fuel < upper_bound_fuel - 1 {
            let next_fuel = (upper_bound_fuel + lower_bound_fuel) / 2;
            let ore_count = make_fuel(next_fuel, &rule_map);
            match ore_count.cmp(&target_ore) {
                Ordering::Less => {
                    assert!(next_fuel >= lower_bound_fuel);
                    lower_bound_fuel = next_fuel;
                }
                Ordering::Equal => {
                    println!("early return!");
                    return next_fuel;
                }
                Ordering::Greater => {
                    assert!(next_fuel <= upper_bound_fuel);
                    upper_bound_fuel = next_fuel;
                }
            }
        }

        assert!(upper_bound_fuel - lower_bound_fuel <= 1);

        lower_bound_fuel
    }
}

fn make_fuel<'a>(fuel_count: usize, rule_map: &HashMap<&'a str, Rule<'a>>) -> usize {
    let mut ore_count = 0;
    let mut needed: HashMap<&str, usize> = HashMap::new();
    let mut excess: HashMap<&str, usize> = HashMap::new();
    needed.insert("FUEL", fuel_count);

    while !needed.is_empty() {
        let key = &needed.keys().next().unwrap().clone();
        let mut needed_count = needed.remove(*key).unwrap();

        let excess_entry = excess.entry(key).or_default();
        if *excess_entry > 0 {
            match (*excess_entry).cmp(&needed_count) {
                Ordering::Greater | Ordering::Equal => {
                    *excess_entry -= needed_count;
                    continue;
                }
                Ordering::Less => {
                    needed_count -= *excess_entry;
                    *excess_entry = 0;
                }
            }
        }

        let Rule {
            inputs,
            output: (output_count, output_compound),
        } = &rule_map[*key];
        assert_eq!(output_compound, key);

        let mult = (needed_count as f32 / *output_count as f32).ceil() as usize;

        if output_count * mult > needed_count {
            *excess.entry(output_compound).or_default() += (output_count * mult) - needed_count;
        }

        for (input_count, input_compound) in inputs.iter() {
            let mut needed_input_count = input_count * mult;

            let excess_entry = excess.entry(&input_compound).or_default();
            if *excess_entry > 0 {
                match (*excess_entry).cmp(&needed_input_count) {
                    Ordering::Greater | Ordering::Equal => {
                        *excess_entry -= needed_input_count;
                        continue;
                    }
                    Ordering::Less => {
                        needed_input_count -= *excess_entry;
                        *excess_entry = 0;
                    }
                }
            }

            if *input_compound == "ORE" {
                ore_count += needed_input_count;
            } else {
                let entry = needed.entry(input_compound).or_default();
                *entry += needed_input_count;
            }
        }
    }

    ore_count
}

fn parse_input<'a>(input: &'a String) -> HashMap<&'a str, Rule<'a>> {
    input
        .trimmed_lines()
        .map(|line| {
            let rule = Rule::from_str(line).unwrap();
            (rule.output.1, rule)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    inputs: Vec<(usize, &'a str)>,
    output: (usize, &'a str),
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Result<Self, std::num::ParseIntError> {
        let parts: Vec<_> = s.split("=>").collect();
        assert_eq!(parts.len(), 2);

        let inputs = parts[0]
            .split(',')
            .map(|input_desc| {
                let input_parts: Vec<_> = input_desc.trim().split(' ').collect();
                assert_eq!(input_parts.len(), 2);
                let count: usize = input_parts[0].parse()?;
                Ok((count, input_parts[1]))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let output_parts: Vec<_> = parts[1].trim().split(' ').collect();
        assert_eq!(output_parts.len(), 2);
        let output = (output_parts[0].parse()?, output_parts[1]);

        Ok(Self { inputs, output })
    }
}
