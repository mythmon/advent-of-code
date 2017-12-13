use std::iter::Iterator;

fn main() {
    let input: &'static str = include_str!("input");
    println!("{}", puzzle(input));
}

fn puzzle(input: &str) -> u32 {
    let mut input: Vec<u32> = input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let first = input.first().unwrap().clone();
    input.push(first);

    let mut sum = 0;
    for (&a, &b) in input.iter().zip(&input[1..]) {
        if a == b {
            sum += a;
        }
    }
    sum
}

#[test]
fn examples() {
    assert_eq!(puzzle("1122"), 3);
    assert_eq!(puzzle("1111"), 4);
    assert_eq!(puzzle("1234"), 0);
    assert_eq!(puzzle("91212129"), 9);
}

#[test]
fn correct_solution() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 1141);
}
