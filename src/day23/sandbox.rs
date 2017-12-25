#![feature(match_default_bindings)]
#![feature(slice_patterns)]

extern crate advent;

use advent::day18::{Machine, Instr};

fn main() {
    let input = "
        set b 67
        set c b
        jnz a 2
        jnz 1 5
        mul b 100
        sub b -100000
        set c b
        sub c -17000
    ";

    let instructions: Vec<Instr> = input.trim()
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect();
    let mut machine = Machine::new(0, instructions.clone());
    machine.registers.insert('a', 1);
    machine.run();
    println!("{:?}", machine.registers);
}
