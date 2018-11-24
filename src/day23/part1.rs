#![feature(slice_patterns)]

use crate::day18::{Instr, InstrType, Machine};

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> usize {
    let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    let mut machine = Machine::new(0, instructions.clone());
    machine.run();
    *machine.debug_counts.get(&InstrType::Mul).unwrap()
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 4225);
}

#[test]
fn test_h() {
    let input = get_input();
    let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    let mut machine = Machine::new(0, instructions.clone());
    machine.run();
    assert_eq!(*machine.registers.get(&'h').unwrap_or(&0), 0);
}
