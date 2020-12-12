use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
    twodee::{Dir4, Point, Turn},
};
use std::{error::Error, iter::Iterator, str::FromStr};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input
        .trimmed_lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err: String| err.into())
}

#[derive(Clone, Debug)]
pub enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Forward(i32),
    Left,
    Right,
    Flip,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().next().ok_or("empty string")?;
        let distance: i32 = s[1..]
            .parse()
            .map_err(|err| format!("Can't parse distance {}", err))?;
        match dir {
            'N' => Ok(Self::North(distance)),
            'S' => Ok(Self::South(distance)),
            'E' => Ok(Self::East(distance)),
            'W' => Ok(Self::West(distance)),
            'F' => Ok(Self::Forward(distance)),
            'L' => match distance {
                90 => Ok(Self::Left),
                180 => Ok(Self::Flip),
                270 => Ok(Self::Right),
                _ => Err(format!("Unexpected left turn amount {}", distance)),
            },
            'R' => match distance {
                90 => Ok(Self::Right),
                180 => Ok(Self::Flip),
                270 => Ok(Self::Left),
                _ => Err(format!("Unexpected right turn amount {}", distance)),
            },
            _ => Err(format!("Unexpected direction {}", dir)),
        }
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Instruction>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D12-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case("Example", "F10\nN3\nF7\nR90\nF11", 25)?
            .transformed_case("Solution", include_str!("input"), 1687)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut pos = Point::new(0, 0);
        let mut direction = Dir4::Right;

        for step in input {
            match step {
                Instruction::North(d) => pos += Dir4::Up * d,
                Instruction::South(d) => pos += Dir4::Down * d,
                Instruction::East(d) => pos += Dir4::Right * d,
                Instruction::West(d) => pos += Dir4::Left * d,
                Instruction::Forward(d) => pos += direction * d,
                Instruction::Left => direction *= Turn::Ccw,
                Instruction::Right => direction *= Turn::Cw,
                Instruction::Flip => direction *= Turn::Flip,
            }
        }

        Ok(pos.manhattan_magnitude())
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Instruction>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D12-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case("Example", "F10\nN3\nF7\nR90\nF11", 286)?
            .transformed_case("Solution", include_str!("input"), 20_873)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        // absolute
        let mut pos = Point::zero();
        // relative
        let mut waypoint = Point::new(10, -1);

        for step in input {
            match step {
                Instruction::North(d) => waypoint += Dir4::Up * d,
                Instruction::South(d) => waypoint += Dir4::Down * d,
                Instruction::East(d) => waypoint += Dir4::Right * d,
                Instruction::West(d) => waypoint += Dir4::Left * d,
                Instruction::Left => waypoint *= Turn::Ccw,
                Instruction::Right => waypoint *= Turn::Cw,
                Instruction::Flip => waypoint *= Turn::Flip,
                Instruction::Forward(d) => pos += waypoint * d,
            }
        }

        Ok(pos.manhattan_magnitude())
    }
}
