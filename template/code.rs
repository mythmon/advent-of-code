fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> u32 {
    0
}

#[test]
fn test_example() {
    let input = "";
    assert_eq!(puzzle(input), 42);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    // assert_eq!(puzzle(input), 42);
}
