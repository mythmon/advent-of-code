#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    InGroup(u32),
    Garbage,
    Cancel,
}
