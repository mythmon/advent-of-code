use std::str;

pub trait StringAdventExt {
    fn trimmed_lines(&self) -> TrimmedLines;
}

impl StringAdventExt for &str {
    fn trimmed_lines(&self) -> TrimmedLines {
        TrimmedLines(self.lines())
    }
}

#[derive(Debug, Clone)]
pub struct TrimmedLines<'a>(str::Lines<'a>);

impl<'a> Iterator for TrimmedLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        while let Some(l) = self.0.next() {
            let trimmed = l.trim();
            if trimmed != "" {
                return Some(trimmed);
            }
        }
        None
    }
}
