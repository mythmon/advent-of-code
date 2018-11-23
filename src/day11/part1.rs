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
        .trim()
        .split(",")
        .filter_map(|p| p.trim().parse().ok())
        .fold(HexVec::zero(), |a, b: HexDir| a + b)
        .size() as u32
}

#[test]
fn test_example_1() {
    let input = "ne,ne,ne";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_example_2() {
    let input = "ne,ne,sw,sw";
    assert_eq!(puzzle(input), 0);
}

#[test]
fn test_example_3() {
    let input = "ne,ne,s,s";
    assert_eq!(puzzle(input), 2);
}

#[test]
fn test_example_4() {
    let input = "se,sw,se,sw,sw";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 812);
}
