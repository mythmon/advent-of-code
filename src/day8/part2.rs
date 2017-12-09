extern crate advent;

use std::collections::HashMap;
use std::cmp;
use advent::day8::{Instruction, Operation};

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

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 7491);
}

#[test]
fn test_dec_can_affect_highest_ever() {
    // dec can raise the value of a register, which should affect
    // highest ever.
    let input = parse_input("a dec -1 if a == 0");
    assert_eq!(puzzle(input), 1);
}
