use crate::day08::{Instruction, Operation};
use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use indoc::indoc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Instruction>;
    type Output = isize;

    fn name(&self) -> String {
        "2017-D08-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.lines().map(|l| l.parse().unwrap()).collect())
            .transformed_case(
                "Example",
                indoc!(
                    "
                    b inc 5 if a > 1
                    a inc 1 if b < 5
                    c dec -10 if a >= 1
                    c inc -20 if c == 10"
                ),
                1,
            )?
            .transformed_case("Solution", include_str!("input"), 5_221)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut registers = HashMap::new();
        for instr in input {
            let condition_target = *registers.get(&instr.condition.register).unwrap_or(&0);
            if instr.condition.matches(condition_target) {
                match instr.op {
                    Operation::Inc => *registers.entry(instr.register).or_insert(0) += instr.amount,
                    Operation::Dec => *registers.entry(instr.register).or_insert(0) -= instr.amount,
                }
            }
        }
        *registers.values().max().unwrap()
    }
}
