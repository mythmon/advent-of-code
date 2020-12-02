use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use rayon::prelude::*;
use std::{collections::HashMap, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<String>;
    type Output = usize;

    fn name(&self) -> String {
        "2015-D05-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            /* spell-checker: disable */
            .transformed_case("Example 1", "ugknbfddgicrmopn", 1)
            .transformed_case("Example 2", "aaa", 1)
            .transformed_case("Example 3", "jchzalrnumimnmhp", 0)
            .transformed_case("Example 4", "haegwjzuvuyypxyu", 0)
            .transformed_case("Example 5", "dvszwmarrgswjxmb", 0)
            /* spell-checker: enable */
            .transformed_case("Solution", include_str!("input"), 238)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.par_iter().filter(|s| is_nice_v1(s)).count()
    }
}

fn is_nice_v1(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let bad_list = vec![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

    let mut vowel_count = 0;
    let mut has_dupe = false;

    let chars: Vec<char> = s.chars().collect();
    for window in chars.windows(2) {
        assert_eq!(window.len(), 2);
        let c1 = window[0];
        let c2 = window[1];
        if c1 == c2 {
            has_dupe = true
        }
        if vowels.contains(&c1) {
            vowel_count += 1;
        }
        if bad_list.contains(&(c1, c2)) {
            return false;
        }
    }
    if vowels.contains(chars.last().unwrap()) {
        vowel_count += 1;
    }

    has_dupe && (vowel_count >= 3)
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<String>;
    type Output = usize;

    fn name(&self) -> String {
        "2015-D05-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            /* spell-checker: disable */
            .transformed_case("Example 1", "qjhvhtzxzqqjkmpb", 1)
            .transformed_case("Example 2", "xxyxx", 1)
            .transformed_case("Example 3", "uurcxstgmygtbstg", 0)
            .transformed_case("Example 4", "ieodomkazucvgmuy", 0)
            .transformed_case("Test 1", "sknufchjdvccccta", 1)
            /* spell-checker: enable */
            .transformed_case("Solution", include_str!("input"), 69)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input.par_iter().filter(|s| is_nice_v2(s)).count()
    }
}

fn is_nice_v2(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();

    let alternate = chars.windows(3).any(|window| window[0] == window[2]);

    let pair_repeat = chars
        // Take pairs of characters
        .windows(2)
        // annotate them with the index of the left character
        .enumerate()
        // and build a mapping of pairs to where they occur in the string
        .fold(HashMap::new(), |mut map: HashMap<_, Vec<usize>>, (idx, pair)| {
            map.entry(pair)
                .and_modify(|l| l.push(idx))
                .or_insert_with(|| vec![idx]);
            map
        })
        // and then find any pairs that occurred more than twice
        .values()
        .filter(|l| l.len() >= 2)
        // and have a separation of at least 2 in a pair of elements
        .any(|l| l.last().unwrap() - l[0] > 1);

    alternate && pair_repeat
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .trimmed_lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
