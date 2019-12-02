use crate::cases::Puzzle;

mod day01;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        day01::get_puzzles(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
