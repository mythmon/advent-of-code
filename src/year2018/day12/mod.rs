use crate::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::collections::{HashMap, HashSet};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = PotsState;
    type Output = i64;

    fn name(&self) -> String {
        "2018-D12-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.parse().unwrap())
            .transformed_case("Example", include_str!("example"), 325)
            .transformed_case("Solution", include_str!("input"), 1_184)
            .collect()
    }

    fn run_puzzle(mut pot_state: Self::Input) -> Self::Output {
        for _ in 0..20 {
            pot_state.step();
        }
        pot_state.answer()
    }
}

trait IndexOob<Idx> {
    type Output;
    fn index_oob(&self, index: Idx) -> Option<&Self::Output>;
}

impl<T> IndexOob<i32> for Vec<T> {
    type Output = T;

    fn index_oob(&self, index: i32) -> Option<&Self::Output> {
        if self.len() > (i32::max_value() as usize) {
            panic!("Vec is too large to safely index_oob with i32");
        }
        if index < 0 || index >= self.len() as i32 {
            None
        } else {
            Some(<Self as std::ops::Index<usize>>::index(
                self,
                index as usize,
            ))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PotsState {
    pots: Vec<bool>,
    rules: HashMap<Vec<bool>, bool>,
    offset: i64,
    iterations: usize,
}

impl PotsState {
    fn step(&mut self) {
        self.offset -= 2;
        let mut next_pots = Vec::with_capacity(self.pots.len() + 4);
        next_pots.resize(self.pots.len() + 4, false);
        for i in -2_i32..(self.pots.len() as i32 + 2) {
            let input: Vec<bool> = (-2_i32..=2_i32)
                .map(|offset| *self.pots.index_oob(i as i32 + offset).unwrap_or(&false))
                .collect();
            let output = self.rules.get(&input).unwrap_or(&false);
            next_pots[(i + 2) as usize] = *output;
        }
        self.pots = next_pots;
        self.trim();
        self.iterations += 1;
    }

    fn answer(&self) -> i64 {
        self.pots
            .iter()
            .enumerate()
            .filter(|(_idx, planted)| **planted)
            .map(|(idx, _planted)| idx as i64 + self.offset as i64)
            .sum()
    }

    fn trim(&mut self) {
        let mut first = None;
        for i in 0..self.pots.len() {
            if self.pots[i] {
                first = Some(i);
                break;
            }
        }
        if first.is_none() {
            return;
        }
        let first = first.unwrap();

        let mut last = first;
        for i in last + 1..self.pots.len() {
            if self.pots[i] {
                last = i
            }
        }

        assert!(first <= i64::max_value() as usize);
        self.offset += first as i64;
        self.pots = self.pots[first..=last].to_owned();
    }

    #[allow(dead_code)]
    fn pots_string(&self) -> String {
        let offset = format!("({:>6}) ", self.offset);
        let display: String = self
            .pots
            .iter()
            .map(|&b| if b { '#' } else { '.' })
            .collect();
        offset + &display
    }
}

#[derive(Debug)]
pub struct PotsError(String);

impl From<std::option::NoneError> for PotsError {
    fn from(_: std::option::NoneError) -> Self {
        Self("Tried to unwrap None".to_owned())
    }
}

impl std::str::FromStr for PotsState {
    type Err = PotsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn char_to_bool(c: char) -> bool {
            match c {
                '.' => false,
                '#' => true,
                _ => panic!(format!("Unexpected bool-char: {}", c)),
            }
        }
        let mut lines = s.lines();

        // initial line looks like:
        //    initial state: #..#.#..##......###...###
        let initial_line = lines.next().unwrap();
        let initial = initial_line
            .split_whitespace()
            .nth(2)
            .unwrap()
            .chars()
            .map(char_to_bool)
            .collect();

        lines.next().unwrap(); // skip a blank line
        let mut rules = HashMap::new();
        for rule_line in lines {
            // rule lines look like:
            //   ##... => .
            let mut parts = rule_line.split(" => ");
            let rule_key: Vec<bool> = parts.next()?.chars().map(char_to_bool).collect();
            let rule_result = char_to_bool(parts.next()?.chars().next()?);
            rules.insert(rule_key, rule_result);
        }

        Ok(Self {
            pots: initial,
            rules,
            offset: 0,
            iterations: 0,
        })
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = PotsState;
    type Output = i64;

    fn name(&self) -> String {
        "2018-D12-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.parse().unwrap())
            .transformed_case("Solution", include_str!("input"), 250_000_000_219)
            .collect()
    }

    fn run_puzzle(mut pots_state: Self::Input) -> Self::Output {
        let mut seen_states = HashSet::new();
        let iteration_target = 50_000_000_000_u64;
        let mut repeated_state = None;
        let mut repeated_after = None;

        // find the first repeat
        for i in 0..iteration_target {
            pots_state.step();
            if !seen_states.insert(pots_state.pots.clone()) {
                repeated_state = Some(pots_state.pots.clone());
                repeated_after = Some(i);
                break;
            }
        }

        let repeated_state = repeated_state.expect("should find a repetition");
        let repeated_after = repeated_after.expect("should find a repetition");
        let offset_before = pots_state.offset;

        // find the stride
        let mut stride = None;
        for i in (repeated_after + 1)..iteration_target {
            pots_state.step();
            if pots_state.pots == repeated_state {
                stride = Some(i - repeated_after);
                break;
            }
        }
        let iteration_stride = stride.expect("should find a stride");
        let offset_stride = (pots_state.offset - offset_before) / iteration_stride as i64;

        let iterations_needed = (iteration_target - repeated_after) % iteration_stride;
        for _ in 0..iterations_needed {
            pots_state.step()
        }

        // To account for a "sliding" repetition, increase the offset
        pots_state.offset +=
            offset_stride as i64 * (iteration_target as i64 - pots_state.iterations as i64);
        pots_state.answer()
    }
}
