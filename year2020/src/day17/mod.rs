use advent_lib::cases::Puzzle;

mod part1;
mod part2;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(part1::Runner), Box::new(part2::Runner)]
}
