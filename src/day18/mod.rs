use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
pub struct Machine {
    instructions: Vec<Instr>,
    pub registers: HashMap<char, i64>,
    last_sound: Option<i64>,
    program_counter: usize,
    pub debug_counts: HashMap<InstrType, usize>,
}

impl Machine {
    pub fn new(id: i64, instructions: Vec<Instr>) -> Self {
        let mut reg = HashMap::new();
        reg.insert('p', id);
        Self {
            instructions: instructions,
            registers: reg,
            last_sound: None,
            program_counter: 0,
            debug_counts: HashMap::new(),
        }
    }

    pub fn step(&mut self, input: &mut VecDeque<i64>) -> (bool, Option<i64>) {
        let mut rv = (false, None);
        let instr = self.instructions[self.program_counter];
        match instr {
            Instr { itype: InstrType::Snd, arg1: Some(a), .. } => {
                rv.1 = Some(self.value(&a));
            }
            Instr { itype: InstrType::Set, register: Some(r), arg1: Some(a), .. } => {
                let v = self.value(&a);
                self.registers.insert(r, v);
            }
            Instr { itype: InstrType::Add, register: Some(r), arg1: Some(a), .. } => {
                *(self.registers.entry(r).or_insert(0)) += self.value(&a);
            }
            Instr { itype: InstrType::Sub, register: Some(r), arg1: Some(a), .. } => {
                *(self.registers.entry(r).or_insert(0)) -= self.value(&a);
            }
            Instr { itype: InstrType::Mul, register: Some(r), arg1: Some(a), .. } => {
                *(self.registers.entry(r).or_insert(0)) *= self.value(&a);
            }
            Instr { itype: InstrType::Mod, register: Some(r), arg1: Some(a), .. } => {
                *(self.registers.entry(r).or_insert(0)) %= self.value(&a);
            }
            Instr { itype: InstrType::Rcv, register: Some(r) , .. } => {
                if let Some(v) = input.pop_front() {
                    self.registers.insert(r, v);
                } else {
                    // we are blocked now
                    rv.0 = true;
                    // it will be incremented at the end
                    self.program_counter -= 1;
                }
            }
            Instr { itype: InstrType::Jgz, arg1: Some(a1), arg2: Some(a2), .. } => {
                if self.value(&a1) > 0 {
                    // subtract 1 since 1 will be added at the end of the loop
                    self.program_counter =
                        ((self.program_counter as i64) + self.value(&a2) - 1) as usize;
                }
            }
            Instr { itype: InstrType::Jnz, arg1: Some(a1), arg2: Some(a2), .. } => {
                if self.value(&a1) != 0 {
                    // subtract 1 since 1 will be added at the end of the loop
                    self.program_counter =
                        ((self.program_counter as i64) + self.value(&a2) - 1) as usize;
                }
            }
            _ => panic!(format!("malformed instruction! {:?}", instr)),
        }

        *(self.debug_counts.entry(instr.itype).or_insert(0)) += 1;
        self.program_counter += 1;
        rv
    }

    pub fn run(&mut self) {
        let l =self.instructions.len();
        let mut input = VecDeque::new();
        while (0..l).contains(self.program_counter) {
            let (blocked, _) = self.step(&mut input);
            if blocked {
                break;
            }
        }
    }

    pub fn run_until_blocked(&mut self, input: &mut VecDeque<i64>) -> VecDeque<i64> {
        let mut output = VecDeque::new();
        loop {
            let (blocked, result) = self.step(input);
            if let Some(v) = result {
                output.push_back(v);
            }
            assert!(self.program_counter < self.instructions.len());
            if blocked {
                break;
            }
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
pub struct Instr {
    itype: InstrType,
    register: Option<char>,
    arg1: Option<Arg>,
    arg2: Option<Arg>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrType {
    Snd,
    Set,
    Add,
    Sub,
    Mul,
    Mod,
    Rcv,
    Jgz,
    Jnz,
}

impl FromStr for Instr {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match &parts[..] {
            ["snd", x] => Ok(Instr {
                itype: InstrType::Snd,
                register: None,
                arg1: Some(x.parse()?),
                arg2: None
            }),
            ["set", x, y] => Ok(Instr {
                itype: InstrType::Set,
                register: Some(x.parse().unwrap()),
                arg1: Some(y.parse().unwrap()),
                arg2: None,
            }),
            ["add", x, y] => Ok(Instr {
                itype: InstrType::Add,
                register: Some(x.parse().unwrap()),
                arg1: Some(y.parse().unwrap()),
                arg2: None,
            }),
            ["sub", x, y] => Ok(Instr {
                itype: InstrType::Sub,
                register: Some(x.parse().unwrap()),
                arg1: Some(y.parse().unwrap()),
                arg2: None,
            }),
            ["mul", x, y] => Ok(Instr {
                itype: InstrType::Mul,
                register: Some(x.parse().unwrap()),
                arg1 : Some(y.parse().unwrap()),
                arg2: None,
            }),
            ["mod", x, y] => Ok(Instr {
                itype: InstrType::Mod,
                register: Some(x.parse().unwrap()),
                arg1: Some(y.parse().unwrap()),
                arg2: None,
            }),
            ["rcv", x] => Ok(Instr {
                itype: InstrType::Rcv,
                register: Some(x.parse().unwrap()),
                arg1: None,
                arg2: None
            }),
            ["jgz", x, y] => Ok(Instr {
                itype: InstrType::Jgz,
                register: None,
                arg1: Some(x.parse().unwrap()),
                arg2: Some(y.parse().unwrap()),
            }),
            ["jnz", x, y] => Ok(Instr {
                itype: InstrType::Jnz,
                register: None,
                arg1: Some(x.parse().unwrap()),
                arg2: Some(y.parse().unwrap()),
            }),
            _ => Err(format!("Could not parse instruction: {}", input)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Arg {
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
