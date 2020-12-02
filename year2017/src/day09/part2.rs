use crate::day09::ParseAction;
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D09-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "<>", 0_u32)
            .case("Example 2", "<random characters>", 17_u32)
            .case("Example 3", "<<<<>", 3_u32)
            .case("Example 4", "<{!>}>", 2_u32)
            .case("Example 5", "<!!>", 0_u32)
            .case("Example 6", "<!!!>>", 0_u32)
            .case("Example 7", "<{o\"i!a,<{i<a>", 10_u32)
            .case("Solution", include_str!("input"), 7_825_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        use crate::day09::ParseState;

        let mut garbage_count = 0;
        let mut state_stack = vec![];

        for c in input.trim().chars() {
            let next_action = match (&state_stack.last(), c) {
                (Some(&s), '!') if s != ParseState::Cancel => ParseAction::Push(ParseState::Cancel),
                (Some(ParseState::Cancel), _)
                | (Some(ParseState::InGroup(_)), '}')
                | (Some(ParseState::Garbage), '>') => ParseAction::Pop,
                (None, '{') => ParseAction::Push(ParseState::InGroup(1)),
                (Some(ParseState::InGroup(v)), '{') => {
                    ParseAction::Push(ParseState::InGroup(v + 1))
                }
                (Some(ParseState::InGroup(_)), ',') => ParseAction::Nothing,
                (Some(ParseState::Garbage), _) => {
                    garbage_count += 1;
                    ParseAction::Nothing
                }
                (_, '<') => ParseAction::Push(ParseState::Garbage),
                (&s, c) => panic!("unexpected input '{}' in {:?}", c, s),
            };

            match next_action {
                ParseAction::Nothing => {}
                ParseAction::Pop => {
                    state_stack.pop();
                }
                ParseAction::Push(v) => {
                    state_stack.push(v);
                }
            }
        }

        assert_eq!(state_stack.len(), 0);

        garbage_count
    }
}
