use advent_lib::cases::Puzzle;

pub mod part1;
pub mod part2;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(part1::Part1), Box::new(part2::Part2)]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    InGroup(u32),
    Garbage,
    Cancel,
}

enum ParseAction {
    Nothing,
    Pop,
    Push(ParseState),
}
