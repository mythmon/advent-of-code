use std::iter::Iterator;

fn main() {
    let input: &'static str = include_str!("input.txt");
    println!("{}", puzzle(input));
}

fn puzzle(input: &str) -> u32 {
    println!("test case {}", input);
    let input: Vec<char> = input.chars().filter(|&c| c >= '0' && c <= '9').collect();
    let offset_input = input.iter().cycle().skip(input.len() / 2);

    let mut sum = 0;
    for (a, b) in input.iter().zip(offset_input) {
        if a == b {
            sum += a.to_digit(10).unwrap();
        }
    }
    sum
}

#[test]
fn examples() {
    assert_eq!(puzzle("1212"), 6);
    assert_eq!(puzzle("1221"), 0);
    assert_eq!(puzzle("123425"), 4);
    assert_eq!(puzzle("123123"), 12);
    assert_eq!(puzzle("12131415"), 4);
}

#[test]
fn correct_solution() {
    let input: &'static str = include_str!("input.txt");
    assert_eq!(puzzle(input), 950);
}
