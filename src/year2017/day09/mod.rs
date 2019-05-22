pub mod part1;
pub mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    InGroup(u32),
    Garbage,
    Cancel,
}

enum ParseAction {
    Nothing,
    Pop,
    Push(ParseState),
}
