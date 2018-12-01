use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    year2017::day08::{Instruction, Operation},
};
use std::{cmp, collections::HashMap};

#[derive(Debug)]
pub struct Day08Part2;

impl PuzzleRunner for Day08Part2 {
    type Input = Vec<Instruction>;
    type Output = isize;

    fn name(&self) -> String {
        "2017-D08-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.lines().map(|l| l.parse().unwrap()).collect())
            .transformed_case("Solution", include_str!("input"), 7_491)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut registers = HashMap::new();
        let mut highest_ever = 0;
        for instr in input {
            let condition_target = *registers.get(&instr.condition.register).unwrap_or(&0);
            if instr.condition.matches(condition_target) {
                let reg = registers.entry(instr.register).or_insert(0);
                match instr.op {
                    Operation::Inc => *reg += instr.amount,
                    Operation::Dec => *reg -= instr.amount,
                }
                highest_ever = cmp::max(highest_ever, *reg);
            }
        }
        highest_ever
    }
}

#[cfg(test)]
mod tests {
    use super::Day08Part2;
    use crate::{cases::PuzzleRunner, year2017::day08::Instruction};

    #[test]
    fn test_dec_can_affect_highest_ever() {
        // dec can raise the value of a register when used with negative
        // numbers, which should affect highest ever.
        let instr: Instruction = "a dec -1 if a == 0".parse().unwrap();
        assert_eq!(Day08Part2::run_puzzle(vec![instr]), 1);
    }
}
