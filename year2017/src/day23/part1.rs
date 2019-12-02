use crate::day18::{Instr, InstrType, Machine};
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D23-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 4_225_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
        let mut machine = Machine::new(0, instructions);
        machine.run();
        machine.debug_counts[&InstrType::Mul]
    }
}

#[test]
fn test_h() {
    let input = include_str!("input");
    let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    let mut machine = Machine::new(0, instructions);
    machine.run();
    assert_eq!(*machine.registers.get(&'h').unwrap_or(&0), 0);
}
