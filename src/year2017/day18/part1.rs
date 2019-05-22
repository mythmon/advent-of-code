use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use indoc::{indoc, indoc_impl};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = i64;

    fn name(&self) -> String {
        "2017-D18-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc!(
                    "
                    set a 1
                    add a 2
                    mul a a
                    mod a 5
                    snd a
                    set a 0
                    rcv a
                    jgz a -1
                    set a 1
                    jgz a -2"
                ),
                4,
            )
            .case("Solution", include_str!("input"), 1_187)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut machine: Machine = input.parse().unwrap();
        machine.run_until_recover()
    }
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instr>,
    registers: HashMap<char, i64>,
    last_sound: Option<i64>,
    program_counter: usize,
}

impl Machine {
    fn run_until_recover(&mut self) -> i64 {
        loop {
            let instr = self.instructions[self.program_counter];
            match instr {
                Instr::Snd(a) => {
                    self.last_sound = Some(self.value(&a));
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
                Instr::Rcv(a) => {
                    if self.value(&a) != 0 {
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
        self.last_sound.unwrap()
    }

    fn value(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Value(v) => *v,
            Arg::Register(r) => *(self.registers.get(r).unwrap_or(&0)),
        }
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self {
            instructions,
            registers: HashMap::new(),
            last_sound: None,
            program_counter: 0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Snd(Arg),
    Set(char, Arg),
    Add(char, Arg),
    Mul(char, Arg),
    Mod(char, Arg),
    Rcv(Arg),
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
