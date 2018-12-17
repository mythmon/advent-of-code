use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day09Part1;

impl PuzzleRunner for Day09Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D09-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "{}", 1_u32)
            .case("Example 2", "{{{}}}", 6_u32)
            .case("Example 3", "{{},{}}", 5_u32)
            .case("Example 4", "{{{},{},{{}}}}", 16_u32)
            .case("Example 5", "{<a>,<a>,<a>,<a>}", 1_u32)
            .case("Example 6", "{{<ab>},{<ab>},{<ab>},{<ab>}}", 9_u32)
            .case("Example 7", "{{<!!>},{<!!>},{<!!>},{<!!>}}", 9_u32)
            .case("Example 8", "{{<a!>},{<a!>},{<a!>},{<ab>}}", 3_u32)
            .case("Solution", include_str!("input"), 17_390_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        use crate::year2017::day09::ParseState::*;

        let mut total_score = 0;
        let mut state_stack = vec![];

        for c in input.trim().chars() {
            let state = state_stack.last();
            match (state, c) {
                (Some(&s), '!') if s != Cancel => {
                    state_stack.push(Cancel);
                }
                (Some(Cancel), _) | (Some(Garbage), '>') => {
                    state_stack.pop();
                }
                (None, '{') => {
                    state_stack.push(InGroup(1));
                }
                (Some(InGroup(v)), '{') => {
                    state_stack.push(InGroup(v + 1));
                }
                (Some(&InGroup(v)), '}') => {
                    state_stack.pop();
                    total_score += v;
                }
                (Some(InGroup(_)), '<') => {
                    state_stack.push(Garbage);
                }
                (Some(InGroup(_)), ',') | (Some(Garbage), _) => (),

                _ => panic!("unexpected input '{}' in {:?}", c, state),
            }
        }

        assert_eq!(state_stack.len(), 0);

        total_score
    }
}
