#![feature(slice_patterns)]

use advent::day18::{Machine, Instr};
use std::collections::VecDeque;

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
    let mut m0 = Machine::new(0, instructions.clone());
    let mut m1 = Machine::new(1, instructions);

    let mut io = VecDeque::new();
    let mut to_m0_queue = m1.run_until_blocked(&mut io);
    let mut m1_send_count = to_m0_queue.len();
    let mut to_m1_queue = m0.run_until_blocked(&mut io);

    loop {
        io = m1.run_until_blocked(&mut to_m1_queue);
        if io.len() == 0 {
            // m0 is blocked on RCV, but m1 didn't SND anything.
            break;
        }
        m1_send_count += io.len();
        to_m0_queue.append(&mut io);

        io = m0.run_until_blocked(&mut to_m0_queue);
        if io.len() == 0 {
            // m1 is blocked on RCV, but m0 didn't SND anything.
            break;
        }
        to_m1_queue.append(&mut io);
    }

    m1_send_count
}

#[test]
fn test_example() {
    let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 5_969);
}
