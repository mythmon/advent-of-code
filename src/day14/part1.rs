use advent::day10::KnotHash;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> u32 {
    (0..128)
        .map(|row| format!("{}-{}", input.trim(), row))
        .flat_map(|row_input| KnotHash::new(&row_input).dense())
        .map(|hash_part| hash_part.count_ones())
        .sum()
}

#[test]
fn test_example() {
    let input = "flqrgnkx";
    assert_eq!(puzzle(input), 8108);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 8148);
}
