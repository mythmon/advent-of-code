#![feature(match_default_bindings)]
#![feature(slice_patterns)]

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

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

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instr>,
    registers: HashMap<char, i64>,
    last_sound: Option<i64>,
    program_counter: usize,
}

impl Machine {
    fn new(id: i64, instructions: Vec<Instr>) -> Self {
        let mut reg = HashMap::new();
        reg.insert('p', id);
        Self {
            instructions: instructions,
            registers: reg,
            last_sound: None,
            program_counter: 0,
        }
    }

    fn run_until_blocked(&mut self, input: &mut VecDeque<i64>) -> VecDeque<i64> {
        let mut output = VecDeque::new();
        loop {
            let instr = self.instructions[self.program_counter];
            match instr {
                Instr::Snd(a) => {
                    output.push_back(self.value(&a));
                }
                Instr::Set(r, a) => {
                    let v = self.value(&a);
                    self.registers.insert(r, v);
                }
                Instr::Add(r, a) => {
                    *(self.registers.entry(r).or_insert(0)) += self.value(&a);
                }
                Instr::Mul(r, a) => {
                    *(self.registers.entry(r).or_insert(0)) *= self.value(&a);
                }
                Instr::Mod(r, a) => {
                    *(self.registers.entry(r).or_insert(0)) %= self.value(&a);
                }
                Instr::Rcv(r) => {
                    if let Some(v) = input.pop_front() {
                        self.registers.insert(r, v);
                    } else {
                        // we are blocked now
                        break;
                    }
                }
                Instr::Jgz(a1, a2) => {
                    if self.value(&a1) > 0 {
                        // subtract 1 since 1 will be added at the end of the loop
                        self.program_counter =
                            ((self.program_counter as i64) + self.value(&a2) - 1) as usize;
                    }
                }
            }

            self.program_counter += 1;
            assert!(self.program_counter < self.instructions.len());
        }
        output
    }

    fn value(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Value(v) => *v,
            Arg::Register(r) => *(self.registers.get(r).unwrap_or(&0)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Snd(Arg),
    Set(char, Arg),
    Add(char, Arg),
    Mul(char, Arg),
    Mod(char, Arg),
    Rcv(char),
    Jgz(Arg, Arg),
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match &parts[..] {
            ["snd", x] => Ok(Instr::Snd(x.parse()?)),
            ["set", x, y] => Ok(Instr::Set(x.parse().unwrap(), y.parse().unwrap())),
            ["add", x, y] => Ok(Instr::Add(x.parse().unwrap(), y.parse().unwrap())),
            ["mul", x, y] => Ok(Instr::Mul(x.parse().unwrap(), y.parse().unwrap())),
            ["mod", x, y] => Ok(Instr::Mod(x.parse().unwrap(), y.parse().unwrap())),
            ["rcv", x] => Ok(Instr::Rcv(x.parse().unwrap())),
            ["jgz", x, y] => Ok(Instr::Jgz(x.parse().unwrap(), y.parse().unwrap())),
            _ => Err(format!("Could not parse instruction: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Arg {
    Value(i64),
    Register(char),
}

impl FromStr for Arg {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.parse::<i64>() {
            Ok(v) => Ok(Arg::Value(v)),
            Err(e) => {
                if input.len() == 1 {
                    Ok(Arg::Register(input.chars().next().unwrap()))
                } else {
                    Err(format!("Could not parse number: '{}'", e))
                }
            }
        }
    }
}


#[test]
fn test_example() {
    let input = "snd 1\nsnd 2\nsnd p\nrcv a\nrcv b\nrcv c\nrcv d";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    // assert_eq!(puzzle(input), 42);
}
