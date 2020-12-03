use crate::day09::ParseAction;
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D09-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "{}", 1_u32)
            .case("Example 2", "{{{}}}", 6_u32)
            .case("Example 3", "{{},{}}", 5_u32)
            .case("Example 4", "{{{},{},{{}}}}", 16_u32)
            .case("Example 5", "{<a>,<a>,<a>,<a>}", 1_u32)
            .case("Example 6", "{{<ab>},{<ab>},{<ab>},{<ab>}}", 9_u32)
            .case("Example 7", "{{<!!>},{<!!>},{<!!>},{<!!>}}", 9_u32)
            .case("Example 8", "{{<a!>},{<a!>},{<a!>},{<ab>}}", 3_u32)
            .case("Solution", include_str!("input"), 17_390_u32)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        use crate::day09::ParseState;

        let mut total_score = 0;
        let mut state_stack = vec![];

        for c in input.trim().chars() {
            let next_action = match (state_stack.last(), c) {
                (Some(&s), '!') if s != ParseState::Cancel => ParseAction::Push(ParseState::Cancel),
                (Some(ParseState::Cancel), _) | (Some(ParseState::Garbage), '>') => {
                    ParseAction::Pop
                }
                (None, '{') => ParseAction::Push(ParseState::InGroup(1)),
                (Some(ParseState::InGroup(v)), '{') => {
                    ParseAction::Push(ParseState::InGroup(v + 1))
                }
                (Some(&ParseState::InGroup(v)), '}') => {
                    total_score += v;
                    ParseAction::Pop
                }
                (Some(ParseState::InGroup(_)), '<') => ParseAction::Push(ParseState::Garbage),
                (Some(ParseState::InGroup(_)), ',') | (Some(ParseState::Garbage), _) => {
                    ParseAction::Nothing
                }

                (s, c) => panic!("unexpected input '{}' in {:?}", c, s),
            };

            match next_action {
                ParseAction::Nothing => {}
                ParseAction::Push(v) => {
                    state_stack.push(v);
                }
                ParseAction::Pop => {
                    state_stack.pop();
                }
            }
        }

        assert_eq!(state_stack.len(), 0);

        total_score
    }
}
