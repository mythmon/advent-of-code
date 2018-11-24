use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::HashMap;
use std::fmt;

pub struct Day25;

impl PuzzleRunner for Day25 {
    type Input = usize;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D25".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", 12_523_873, 4_225)
            .collect()
    }

    fn run_puzzle(_input: Self::Input) -> Self::Output {
        let mut machine = puzzle_machine();
        machine.run(12523873);
        machine.diagnostics()
    }
}

fn puzzle_machine() -> TuringMachine {
    TuringMachine::new('A')
        .add_rule(('A', '0'), ('1', Dir::Right, 'B'))
        .add_rule(('A', '1'), ('1', Dir::Left, 'E'))
        .add_rule(('B', '0'), ('1', Dir::Right, 'C'))
        .add_rule(('B', '1'), ('1', Dir::Right, 'F'))
        .add_rule(('C', '0'), ('1', Dir::Left, 'D'))
        .add_rule(('C', '1'), ('0', Dir::Right, 'B'))
        .add_rule(('D', '0'), ('1', Dir::Right, 'E'))
        .add_rule(('D', '1'), ('0', Dir::Left, 'C'))
        .add_rule(('E', '0'), ('1', Dir::Left, 'A'))
        .add_rule(('E', '1'), ('0', Dir::Right, 'D'))
        .add_rule(('F', '0'), ('1', Dir::Right, 'A'))
        .add_rule(('F', '1'), ('1', Dir::Right, 'C'))
}

struct TuringMachine {
    state: char,
    rules: HashMap<(char, char), (char, Dir, char)>,
    tape: HashMap<isize, char>,
    pos: isize,
}

impl TuringMachine {
    fn new(initial_state: char) -> Self {
        Self {
            state: initial_state,
            rules: HashMap::new(),
            tape: HashMap::new(),
            pos: 0,
        }
    }

    fn add_rule(mut self, condition: (char, char), result: (char, Dir, char)) -> Self {
        self.rules.insert(condition, result);
        self
    }

    fn run(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.tick();
        }
    }

    fn diagnostics(&mut self) -> usize {
        self.tape.values().filter(|&v| *v == '1').count()
    }

    fn tick(&mut self) {
        let tape = *self.tape.get(&self.pos).unwrap_or(&'0');
        let &(next_tape, dir, next_state) = self.rules.get(&(self.state, tape)).unwrap();
        self.tape.insert(self.pos, next_tape);
        self.state = next_state;
        match dir {
            Dir::Left => self.pos -= 1,
            Dir::Right => self.pos += 1,
        }
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_pos = *self.tape.keys().min().unwrap_or(&0);
        let max_pos = *self.tape.keys().max().unwrap_or(&min_pos);

        for pos in min_pos..=max_pos {
            let v = self.tape.get(&pos).unwrap_or(&'0');
            if pos == self.pos {
                write!(f, "[{}]", v)?;
            } else {
                write!(f, " {} ", v)?;
            }
        }
        write!(f, "  (state {})", self.state)?;

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

#[test]
fn test_example() {
    let mut machine = TuringMachine::new('A')
        .add_rule(('A', '0'), ('1', Dir::Right, 'B'))
        .add_rule(('A', '1'), ('0', Dir::Left, 'B'))
        .add_rule(('B', '0'), ('1', Dir::Left, 'A'))
        .add_rule(('B', '1'), ('1', Dir::Right, 'A'));

    for pos in -3..3 {
        machine.tape.insert(pos, '0');
    }

    for _ in 0..6 {
        machine.tick();
    }

    assert_eq!(machine.diagnostics(), 3);
}
