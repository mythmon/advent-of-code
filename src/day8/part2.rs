use std::str::FromStr;
use std::collections::HashMap;
use std::cmp;

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
            match instr.op {
                Operation::Inc => {
                    let reg = registers.entry(instr.register).or_insert(0);
                    *reg += instr.amount;
                    highest_ever = cmp::max(highest_ever, *reg);
                },
                Operation::Dec => *registers.entry(instr.register).or_insert(0) -= instr.amount,
            }
        }
    }
    highest_ever
}

struct Instruction {
    register: String,
    op: Operation,
    amount: isize,
    condition: Condition,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 7 {
            return Err(());
        }

        // c inc -20 if c == 10
        Ok(Self {
            register: parts[0].into(),
            op: parts[1].parse()?,
            amount: parts[2].parse().map_err(|_| ())?,
            condition: Condition::from_parts(&parts[4..])?,
        })
    }
}

enum Operation {
    Inc,
    Dec,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(()),
        }
    }
}

struct Condition {
    register: String,
    comparison: Comparison,
    value: isize,
}

impl Condition {
    fn from_parts(parts: &[&str]) -> Result<Self, ()> {
        assert_eq!(parts.len(), 3);
        Ok(Self {
            register: parts[0].into(),
            comparison: parts[1].parse()?,
            value: parts[2].parse().map_err(|_| ())?,
        })
    }

    fn matches(&self, register_value: isize) -> bool {
        match self.comparison {
            Comparison::LessThan => register_value < self.value,
            Comparison::LessThanEqual => register_value <= self.value,
            Comparison::GreaterThan => register_value > self.value,
            Comparison::GreaterThanEqual => register_value >= self.value,
            Comparison::Equal => register_value == self.value,
            Comparison::NotEqual => register_value != self.value,
        }
    }
}

enum Comparison {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
}

impl FromStr for Comparison {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "<" => Ok(Comparison::LessThan),
            "<=" => Ok(Comparison::LessThanEqual),
            ">" => Ok(Comparison::GreaterThan),
            ">=" => Ok(Comparison::GreaterThanEqual),
            "==" => Ok(Comparison::Equal),
            "!=" => Ok(Comparison::NotEqual),
            _ => Err(()),
        }
    }
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 7491);
}
