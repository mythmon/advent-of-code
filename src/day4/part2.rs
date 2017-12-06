use std::collections::{HashSet, HashMap};

fn main() {
    let input: &'static str = include_str!("input");
    println!("{}", puzzle(input));
}

fn puzzle(input: &str) -> usize {
    input.lines().filter(|p| is_valid(p)).count()
}

fn is_valid(passphrase: &str) -> bool {
    let mut seen_words: HashSet<&str> = HashSet::new();
    let words: Vec<&str> = passphrase.split(" ").collect();

    for (idx, word1) in words.iter().enumerate() {
        if seen_words.contains(word1) {
            return false;
        }
        seen_words.insert(word1);
        for word2 in &words[(idx+1)..] {
            if is_anagram(word1, word2) {
                return false;
            }
        }
    }

    true
}

fn is_anagram(a: &str, b: &str) -> bool {
    letter_counts(a) == letter_counts(b)
}

fn letter_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        let mut entry = counts.entry(c).or_insert(0);
        *entry += 1;
    }
    counts
}


#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 251);
}

#[test]
fn test_valid_example_1() {
    assert!(is_valid("abcde fghij"));
}

#[test]
fn test_valid_example_2() {
    assert!(!is_valid("abcde xyz ecdab"));
}

#[test]
fn test_valid_example_3() {
    assert!(is_valid("a ab abc abd abf abj"));
}

#[test]
fn test_valid_example_4() {
    assert!(is_valid("iiii oiii ooii oooi oooo"));
}

#[test]
fn test_valid_example_5() {
    assert!(!is_valid("oiii ioii iioi iiio"));
}

#[test]
fn test_puzzle_example() {
    let input = "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi oooo\noiii ioii iioi iiio";
    assert_eq!(puzzle(input), 3);
}
