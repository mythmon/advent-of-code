#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    // Not useful here
    clippy::filter_map,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,

    clippy::use_self, // doesn't work well with generics
)]
#![feature(str_split_once, once_cell)]

use advent_lib::cases::Puzzle;

mod day01;
mod day02;
mod day03;
mod day04;

#[must_use]
pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        day01::get_puzzles(),
        day02::get_puzzles(),
        day03::get_puzzles(),
        day04::get_puzzles(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
