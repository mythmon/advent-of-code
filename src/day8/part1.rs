extern crate advent;

use advent::day8::{Instruction, Operation};
use std::collections::HashMap;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> Vec<Instruction> {
    let input: &'static str = include_str!("input");
    parse_input(input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn puzzle(input: Vec<Instruction>) -> isize {
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

#[test]
fn test_example() {
    let input = "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10";
    let input = parse_input(input);
    assert_eq!(puzzle(input), 1);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 5221);
}
