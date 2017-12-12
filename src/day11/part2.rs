extern crate advent;

use advent::day11::{HexDir, HexVec};

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> u32 {
    input
        .split(",")
        .filter_map(|p| p.trim().parse().ok())
        .scan(HexVec::zero(), |a, b: HexDir| {
            *a += b;
            Some(a.size())
        })
        .max()
        .unwrap() as u32
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 1603);
}
