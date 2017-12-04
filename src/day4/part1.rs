use std::collections::HashSet;

fn main() {
    let input: &'static str = include_str!("input.txt");
    println!("{}", puzzle(input));
}

fn puzzle(input: &str) -> usize {
    input.lines().filter(|p| is_valid(p)).count()
}

fn is_valid(passphrase: &str) -> bool {
    let mut seen_words = HashSet::new();
    for word in passphrase.split(" ") {
        if seen_words.contains(word) {
            return false;
        } else {
            seen_words.insert(word);
        }
    }
    true
}

#[test]
fn test_is_valid_example_1() {
    let passphrase = "aa bb cc dd ee";
    assert!(is_valid(passphrase));
}

#[test]
fn test_is_valid_example_2() {
    let passphrase = "aa bb cc dd aa";
    assert!(!is_valid(passphrase));
}

#[test]
fn test_is_valid_example_3() {
    let passphrase = "aa bb cc dd aaa";
    assert!(is_valid(passphrase));
}

#[test]
fn test_puzzle_example() {
    let input = "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa";
    assert_eq!(puzzle(input), 2);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input.txt");
    assert_eq!(puzzle(input), 466);
}
