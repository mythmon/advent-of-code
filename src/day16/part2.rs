
#[macro_use] extern crate lazy_static;
extern crate regex;

use std::str::FromStr;
use regex::Regex;

fn main() {
    let input = get_input();
    println!("{}", puzzle(16, input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input.trim()
}

fn puzzle(num_dancers: usize, input: &str) -> String {
    let mut dancers: Vec<u8> = (b'a' ..= b'z').take(num_dancers).collect();
    let original_dancers = dancers.clone();
    let instructions: Vec<Instruction> = input.split(",").map(|p| p.parse().unwrap()).collect();

    let mut cycle_at = None;

    for instr in instructions.iter() {
        instr.exec(&mut dancers);
    }

    for i in 1..1_000_000_000 {
        if dancers == original_dancers {
            cycle_at = Some(i);
            break;
        }
        for instr in instructions.iter() {
            instr.exec(&mut dancers);
        }
    }

    if let Some(cycle_at) = cycle_at {
        dancers = original_dancers.clone();
        for _ in 0..(1_000_000_000 % cycle_at) {
            for instr in instructions.iter() {
                instr.exec(&mut dancers);
            }
        }
    }

    String::from_utf8(dancers).unwrap()
}

enum Instruction {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl Instruction {
    fn exec(&self, dancers: &mut [u8]) {
        let l = dancers.len();
        match self {
            &Instruction::Spin(x) => {
                dancers.rotate_left(l - x);
            },
            &Instruction::Exchange(a_idx, b_idx) => {
                dancers.swap(a_idx, b_idx);
            },
            &Instruction::Partner(a, b) => {
                let a_idx = dancers.iter().position(|&d| d == a).unwrap();
                let b_idx = dancers.iter().position(|&d| d == b).unwrap();
                dancers.swap(a_idx, b_idx);
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new(r"^s(\d+)$").unwrap();
            static ref EXCHANGE_RE: Regex = Regex::new(r"^x(\d+)/(\d+)$").unwrap();
            static ref PARTNER_RE: Regex = Regex::new(r"^p([a-z])/([a-z])$").unwrap();
        }

        if let Some(captures) = SPIN_RE.captures(input) {
            Ok(Instruction::Spin(captures[1].parse().unwrap()))
        } else if let Some(captures) = EXCHANGE_RE.captures(input) {
            Ok(Instruction::Exchange(captures[1].parse().unwrap(), captures[2].parse().unwrap()))
        } else if let Some(captures) = PARTNER_RE.captures(input) {
            Ok(Instruction::Partner(captures[1].as_bytes()[0], captures[2].as_bytes()[0]))
        } else {
            Err(format!("no regexes matched {:?}", input))
        }
    }
}

#[test]
fn test_example_series() {
    let mut dancers: Vec<u8> = "abcde".bytes().collect();
    Instruction::Spin(1).exec(&mut dancers);
    assert_eq!(dancers, "eabcd".bytes().collect::<Vec<u8>>());
    Instruction::Exchange(3, 4).exec(&mut dancers);
    assert_eq!(dancers, "eabdc".bytes().collect::<Vec<u8>>());
    Instruction::Partner(b'e', b'b').exec(&mut dancers);
    assert_eq!(dancers, "baedc".bytes().collect::<Vec<u8>>());
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(16, input);
    // assert_eq!(puzzle(input), 42);
}
