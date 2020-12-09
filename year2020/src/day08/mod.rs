use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::{collections::HashSet, error::Error, iter::Iterator, str::FromStr, sync::mpsc::channel};
use rayon::prelude::*;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    op: Operation,
    data: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, data) = s
            .split_once(' ')
            .ok_or(format!("Syntax error: no space in `{}`", s))?;
        let op = op.parse()?;
        let data = data
            .trim_start_matches('+')
            .parse()
            .map_err(|err| format!("Syntax error, invalid data: `{}`", err))?;

        Ok(Self { op, data })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acc" => Ok(Self::Acc),
            "jmp" => Ok(Self::Jmp),
            "nop" => Ok(Self::Nop),
            _ => Err(format!("Invalid operation `{}`", s)),
        }
    }
}

struct GameConsole {
    program: Vec<Instruction>,
    program_counter: usize,
    accumulator: i32,
}

impl GameConsole {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            program_counter: 0,
            accumulator: 0,
        }
    }

    fn step(&mut self) -> Result<(), String> {
        let instruction = self.program.get(self.program_counter).ok_or(format!(
            "instruction counter out of bounds at {}",
            self.program_counter
        ))?;

        match instruction {
            Instruction {
                op: Operation::Nop, ..
            } => self.program_counter += 1,

            Instruction {
                op: Operation::Acc,
                data,
            } => {
                self.program_counter += 1;
                self.accumulator += *data;
            }

            Instruction {
                op: Operation::Jmp,
                data,
            } => self.program_counter = (self.program_counter as i32 + data) as usize,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Instruction>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D08-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                nop +0
                acc +1
                jmp +4
                acc +3
                jmp -3
                acc -99
                acc +1
                jmp -4
                acc +6
            "},
                5,
            )?
            .transformed_case("Solution", include_str!("input"), 1_766)?
            .collect())
    }

    fn try_run_puzzle(program: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut console = GameConsole::new(program);
        let mut indexes_executed = HashSet::new();

        loop {
            if indexes_executed.contains(&console.program_counter) {
                break;
            }
            indexes_executed.insert(console.program_counter);
            console.step()?;
        }

        Ok(console.accumulator)
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Instruction>;
    type Output = i32;

    fn name(&self) -> String {
        "2020-D08-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example 1",
                indoc! {"
                nop +0
                acc +1
                jmp +4
                acc +3
                jmp -3
                acc -99
                acc +1
                jmp -4
                acc +6
            "},
                8,
            )?
            .transformed_case("Solution", include_str!("input"), 1_639)?
            .collect())
    }

    fn try_run_puzzle(program: Self::Input) -> Result<Self::Output, Self::Error> {
        let iter = (0..program.len())
            .par_bridge()
            .map(|index_to_change| -> Result<Option<i32>, String> {
                let mut changed_program = program.clone();
                changed_program[index_to_change] = match changed_program[index_to_change] {
                    Instruction { op: Operation::Nop, data } => Instruction { op: Operation::Jmp, data },
                    Instruction { op: Operation::Jmp, data } => Instruction { op: Operation::Nop, data },
                    _ => return Ok(None),
                };

                let mut console = GameConsole::new(changed_program);
                let mut indexes_executed = HashSet::new();

                loop {
                    if console.program_counter >= program.len() {
                        return Ok(Some(console.accumulator));
                    }
                    if indexes_executed.contains(&console.program_counter) {
                        return Ok(None);
                    }
                    indexes_executed.insert(console.program_counter);
                    console.step()?;
                }
            });

        let (answer_tx, answer_rx) = channel();
        let (error_tx, error_rx) = channel();

        iter
            .try_for_each_with((answer_tx, error_tx), |(answer_tx, error_tx), candidate| {
                match candidate {
                    Ok(Some(answer)) => { answer_tx.send(answer).unwrap(); Ok(()) },
                    Ok(None) => Ok(()),
                    Err(err) => { error_tx.send(err).unwrap(); Err(()) },
                }
            })
            .map_err(|_| format!("There was an error: {:?}", error_rx.iter().next().unwrap()))?;

        answer_rx.iter().next().ok_or("No answer found".into())
    }
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    Ok(input
        .trimmed_lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?)
}
