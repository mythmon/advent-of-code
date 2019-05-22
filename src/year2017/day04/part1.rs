use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D04-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                "aa bb cc dd ee\naa bb cc dd aa\naa bb cc dd aaa",
                2_usize,
            )
            .case("Solution", include_str!("input"), 466_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.lines().filter(|p| is_valid(p)).count()
    }
}

fn is_valid(passphrase: &str) -> bool {
    let mut seen_words = HashSet::new();
    for word in passphrase.split(' ') {
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
