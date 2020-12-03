use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use std::{collections::HashMap, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Vec<Instruction>>;
    type Output = u64;

    fn name(&self) -> String {
        "2019-D03-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Example 1", "R8,U5,L5,D3\nU7,R6,D4,L4", 6)?
            .transformed_case(
                "Example 2",
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                159,
            )?
            .transformed_case(
                "Example 3",
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            )?
            .transformed_case("Solution", include_str!("input"), 221)?
            .collect())
    }

    fn run_puzzle(routes: Self::Input) -> Self::Output {
        map_wires(&routes)
            .into_iter()
            .filter(|(_, crossings)| crossings[0].is_some() && crossings[1].is_some())
            .map(|((dx, dy), _)| dx.abs() as u64 + dy.abs() as u64)
            .min()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Vec<Instruction>>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D03-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Example 1", "R8,U5,L5,D3\nU7,R6,D4,L4", 30)?
            .transformed_case(
                "Example 2",
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83",
                610,
            )?
            .transformed_case(
                "Example 3",
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                410,
            )?
            .transformed_case("Solution", include_str!("input"), 18_542)?
            .collect())
    }

    fn run_puzzle(routes: Self::Input) -> Self::Output {
        map_wires(&routes)
            .into_iter()
            .filter(|(_, crossings)| crossings[0].is_some() && crossings[1].is_some())
            .map(|(_, crossings)| crossings[0].unwrap() + crossings[1].unwrap())
            .min()
            .unwrap()
    }
}

fn map_wires(routes: &[Vec<Instruction>]) -> HashMap<(i32, i32), [Option<usize>; 2]> {
    let mut world = HashMap::<(i32, i32), [Option<usize>; 2]>::new();

    for (i, wire) in routes.iter().enumerate() {
        let mut pos = (0, 0);
        let mut total_length = 1;
        for inst in wire {
            let (offset, dist): ((i32, i32), usize) = match inst {
                Instruction::Left(dist) => ((-1, 0), *dist),
                Instruction::Right(dist) => ((1, 0), *dist),
                Instruction::Up(dist) => ((0, -1), *dist),
                Instruction::Down(dist) => ((0, 1), *dist),
            };
            for _ in 0..dist {
                pos.0 += offset.0;
                pos.1 += offset.1;
                let e = world.entry(pos).or_insert([None, None]);
                if e[i].is_none() {
                    e[i] = Some(total_length)
                }
                total_length += 1;
            }
        }
    }

    world
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .trimmed_lines()
        .map(|line| {
            line.split(',')
                .map(|instr_str| match instr_str.split_at(1) {
                    ("U", count_str) => Instruction::Up(count_str.parse().unwrap()),
                    ("D", count_str) => Instruction::Down(count_str.parse().unwrap()),
                    ("L", count_str) => Instruction::Left(count_str.parse().unwrap()),
                    ("R", count_str) => Instruction::Right(count_str.parse().unwrap()),
                    _ => panic!(format!("Unexpected instruction {}", instr_str)),
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}
