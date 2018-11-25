use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day09Part2;

impl PuzzleRunner for Day09Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D09-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "<>", 0)
            .case("Example 2", "<random characters>", 17)
            .case("Example 3", "<<<<>", 3)
            .case("Example 4", "<{!>}>", 2)
            .case("Example 5", "<!!>", 0)
            .case("Example 6", "<!!!>>", 0)
            .case("Example 7", "<{o\"i!a,<{i<a>", 10)
            .case("Solution", include_str!("input"), 7_825)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        use crate::day09::ParseState::*;

        let mut garbage_count = 0;
        let mut state_stack = vec![];

        for c in input.trim().chars() {
            let state = state_stack.last();
            match (state, c) {
                (Some(&s), '!') if s != Cancel => {
                    state_stack.push(Cancel);
                }
                (Some(Cancel), _) => {
                    state_stack.pop();
                }
                (None, '{') => {
                    state_stack.push(InGroup(1));
                }
                (Some(InGroup(v)), '{') => {
                    state_stack.push(InGroup(v + 1));
                }
                (Some(InGroup(_)), '}') => {
                    state_stack.pop();
                }
                (Some(InGroup(_)), ',') => (),
                (Some(Garbage), '>') => {
                    state_stack.pop();
                }
                (Some(Garbage), _) => {
                    garbage_count += 1;
                }
                (_, '<') => {
                    state_stack.push(Garbage);
                }

                _ => panic!("unexpected input '{}' in {:?}", c, state),
            }
        }

        assert_eq!(state_stack.len(), 0);

        garbage_count
    }
}
