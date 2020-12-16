use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::{
    collections::HashSet,
    error::Error,
    iter::Iterator,
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug, Clone)]
struct ValidityRange {
    name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

#[derive(Debug, Clone)]
struct Ticket(Vec<u32>);

#[derive(Debug, Clone)]
pub struct PuzzleInput {
    ranges: Vec<ValidityRange>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_input(input: &str) -> Result<PuzzleInput, Box<dyn Error>> {
    let mut iter = input.paragraphs();

    let ranges = iter
        .next()
        .ok_or("No validity ranges found")?
        .trimmed_lines()
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let my_ticket = Ticket(
        iter.next()
            .ok_or("My ticket not found")?
            .trimmed_lines()
            .nth(1)
            .ok_or("no my ticket data")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?,
    );

    let nearby_tickets = iter
        .next()
        .ok_or("No nearby tickets")?
        .trimmed_lines()
        .skip(1)
        .map(|line| -> Result<Ticket, ParseIntError> {
            Ok(Ticket(
                line.split(',').map(str::parse).collect::<Result<_, _>>()?,
            ))
        })
        .collect::<Result<_, _>>()?;

    Ok(PuzzleInput {
        ranges,
        my_ticket,
        nearby_tickets,
    })
}

impl FromStr for ValidityRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, range_strs) = s.split_once(": ").ok_or("no colon")?;
        let ranges = range_strs
            .split(" or ")
            .map(|p| -> Result<_, Self::Err> {
                let range_parts: Vec<u32> = p
                    .split('-')
                    .map(str::parse::<u32>)
                    .collect::<Result<_, _>>()
                    .map_err(|err| err.to_string())?;
                let a = range_parts.get(0).ok_or("no first part")?;
                let b = range_parts.get(1).ok_or("no second part")?;
                Ok(*a..=*b)
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            name: name.to_string(),
            ranges,
        })
    }
}

impl ValidityRange {
    fn contains(&self, field: u32) -> bool {
        self.ranges.iter().any(|range| range.contains(&field))
    }
}

impl Ticket {
    fn never_valid_fields(&self, ranges: &[ValidityRange]) -> Vec<u32> {
        self.0
            .iter()
            .filter(|value| !ranges.iter().any(|range| range.contains(**value)))
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = PuzzleInput;
    type Output = u32;

    fn name(&self) -> String {
        "2020-D16-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                class: 1-3 or 5-7
                row: 6-11 or 33-44
                seat: 13-40 or 45-50

                your ticket:
                7,1,14

                nearby tickets:
                7,3,47
                40,4,50
                55,2,20
                38,6,12
            "},
                71,
            )?
            .transformed_case("Solution", include_str!("input"), 26_009)?
            .collect())
    }

    fn run_puzzle(
        PuzzleInput {
            nearby_tickets,
            ranges,
            ..
        }: PuzzleInput,
    ) -> Self::Output {
        nearby_tickets
            .iter()
            .flat_map(|ticket| ticket.never_valid_fields(&ranges))
            .sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = PuzzleInput;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D16-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                class: 0-1 or 4-19
                departure_row: 0-5 or 8-19
                departure_seat: 0-13 or 16-19

                your ticket:
                11,12,13

                nearby tickets:
                3,9,18
                15,1,5
                5,14,9
            "},
                143,
            )?
            .transformed_case("Solution", include_str!("input"), 589_685_618_167)?
            .collect())
    }

    fn try_run_puzzle(
        PuzzleInput {
            ranges,
            my_ticket,
            nearby_tickets,
        }: Self::Input,
    ) -> Result<Self::Output, Self::Error> {
        let mut possibilities: Vec<HashSet<&str>> = (0..my_ticket.0.len())
            .map(|_idx| ranges.iter().map(|range| range.name.as_str()).collect())
            .collect();

        let valid_tickets: Vec<_> = nearby_tickets
            .iter()
            .filter(|t| t.never_valid_fields(&ranges).is_empty())
            .collect();

        loop {
            for ticket in &valid_tickets {
                for (idx, value) in ticket.0.iter().enumerate() {
                    for range in &ranges {
                        if !range.contains(*value) {
                            possibilities[idx].remove(range.name.as_str());
                        }
                    }
                }
            }

            if possibilities.iter().all(|set| set.len() == 1) {
                break;
            }

            let mut to_remove = Vec::new();
            for (i, set) in possibilities.iter().enumerate() {
                if set.len() == 1 {
                    to_remove.push((i, *set.iter().next().unwrap()));
                }
            }
            for (i, name) in to_remove {
                for j in (0..possibilities.len()).filter(|j| i != *j) {
                    possibilities[j].remove(name);
                }
            }
        }

        let mut result_prod: u64 = 1;
        for (idx, set) in possibilities.into_iter().enumerate() {
            let name = set.into_iter().next().unwrap();
            if name.starts_with("departure") {
                result_prod *= u64::from(my_ticket.0[idx]);
            }
        }

        Ok(result_prod)
    }
}
