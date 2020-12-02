use advent_lib::cases::Puzzle;
use std::{str::FromStr, num::ParseIntError};

pub mod part1;
pub mod part2;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(part1::Part1), Box::new(part2::Part2)]
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub register: String,
    pub op: Operation,
    pub amount: isize,
    pub condition: Condition,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 7 {
            return Err(format!("Expected 7 instruction parts, found {}", parts.len()));
        }

        // c inc -20 if c == 10
        Ok(Self {
            register: parts[0].into(),
            op: parts[1].parse().map_err(|err| format!("Couldn't parse operator: {}", err))?,
            amount: parts[2].parse().map_err(|err: ParseIntError| format!("Could not parse amount: {}", err))?,
            condition: Condition::from_parts(&parts[4..]).map_err(|err| format!("Couldn't parse condition: {}", err))?,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Inc,
    Dec,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(format!("Couldn't parse `{}` as operation", input)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Condition {
    pub register: String,
    pub comparison: Comparison,
    pub value: isize,
}

impl Condition {
    fn from_parts(parts: &[&str]) -> Result<Self, String> {
        assert_eq!(parts.len(), 3);
        Ok(Self {
            register: parts[0].into(),
            comparison: parts[1].parse().map_err(|err| format!("Couldn't parse comparison: {}", err))?,
            value: parts[2].parse().map_err(|err| format!("Couldn't parse value: {}", err))?,
        })
    }

    pub const fn matches(&self, register_value: isize) -> bool {
        match self.comparison {
            Comparison::LessThan => register_value < self.value,
            Comparison::LessThanEqual => register_value <= self.value,
            Comparison::GreaterThan => register_value > self.value,
            Comparison::GreaterThanEqual => register_value >= self.value,
            Comparison::Equal => register_value == self.value,
            Comparison::NotEqual => register_value != self.value,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Comparison {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
}

impl FromStr for Comparison {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "<" => Ok(Comparison::LessThan),
            "<=" => Ok(Comparison::LessThanEqual),
            ">" => Ok(Comparison::GreaterThan),
            ">=" => Ok(Comparison::GreaterThanEqual),
            "==" => Ok(Comparison::Equal),
            "!=" => Ok(Comparison::NotEqual),
            _ => Err(format!("Could not parse `{}` as a comparison", input)),
        }
    }
}
