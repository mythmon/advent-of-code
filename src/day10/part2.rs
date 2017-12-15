extern crate advent;

use advent::day10::KnotHash;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input.trim()
}

fn puzzle(input: &str) -> String {
    KnotHash::new(input).hex()
}

#[test]
fn test_example_1() {
    let input = "";
    assert_eq!(puzzle(input), "a2582a3a0e66e6e86e3812dcb672a272");
}

#[test]
fn test_example_2() {
    let input = "AoC 2017";
    assert_eq!(puzzle(input), "33efeb34ea91902bb2f59c9920caa6cd");
}

#[test]
fn test_example_3() {
    let input = "1,2,3";
    assert_eq!(puzzle(input), "3efbe78a8d82f29979031a4aa0b16a9d");
}

#[test]
fn test_example_4() {
    let input = "1,2,4";
    assert_eq!(puzzle(input), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), "70b856a24d586194331398c7fcfa0aaf");
}
