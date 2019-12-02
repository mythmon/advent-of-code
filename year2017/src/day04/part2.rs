use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D04-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        // spell-checker: disable
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "abcde fghij\nabcde xyz ecdab\na ab abc abd abf abj\niiii oiii ooii oooi \
                 oooo\noiii ioii iioi iiio",
                3_usize,
            )
            .case("Solution", include_str!("input"), 251_usize)
            .collect()
        // spell-checker: enable
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.lines().filter(|p| is_valid(p)).count()
    }
}

fn is_valid(passphrase: &str) -> bool {
    let mut seen_words: HashSet<&str> = HashSet::new();
    let words: Vec<&str> = passphrase.split(' ').collect();

    for (idx, word1) in words.iter().enumerate() {
        if seen_words.contains(word1) {
            return false;
        }
        seen_words.insert(word1);
        for word2 in &words[(idx + 1)..] {
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
        let entry = counts.entry(c).or_insert(0);
        *entry += 1;
    }
    counts
}

#[test]
fn test_valid_example_1() {
    assert!(is_valid("abcde fghij")); // spell-checker: disable-line
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
    assert!(is_valid("iiii oiii ooii oooi oooo")); // spell-checker: disable-line
}

#[test]
fn test_valid_example_5() {
    assert!(!is_valid("oiii ioii iioi iiio")); // spell-checker: disable-line
}
