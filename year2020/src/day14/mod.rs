use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    iter::{repeat, Iterator},
    str::FromStr,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn parse_input(input: &str) -> Result<Vec<Assignment>, Box<dyn Error>> {
    input
        .trimmed_lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err: String| err.into())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Num36(Vec<bool>);

impl Num36 {
    fn with_mask(&self, mask: &Mask36) -> Self {
        Self(
            mask.0
                .iter()
                .zip(self.0.iter())
                .map(|(mask_bit, self_bit)| mask_bit.unwrap_or(*self_bit))
                .collect(),
        )
    }
}

impl Mask36 {
    fn modify_address(&self, address: u64) -> Vec<u64> {
        let mut rv = vec![address];
        for (idx, bit) in self.0.iter().enumerate() {
            let bit_num = 35 - idx;
            match bit {
                Some(false) => {}
                Some(true) => {
                    for a in &mut rv {
                        *a |= 1 << bit_num
                    }
                }
                None => {
                    rv = rv
                        .iter()
                        .flat_map(|a| {
                            let m = 1 << bit_num;
                            vec![a | m, a & (!m)]
                        })
                        .collect();
                }
            }
        }
        rv
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mask36(Vec<Option<bool>>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Assignment {
    Mem { address: u64, value: Num36 },
    Mask { mask: Mask36 },
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(" = ")
            .ok_or_else::<String, _>(|| "No equals sign".to_string())?;

        if left == "mask" {
            Ok(Assignment::Mask {
                mask: right.parse()?,
            })
        } else if left.starts_with("mem[") && left.ends_with(']') {
            let address = left
                .strip_prefix("mem[")
                .and_then(|s| s.strip_suffix(']'))
                .ok_or_else(|| "Syntax: Could not parse mem address".to_string())?
                .parse::<u64>()
                .map_err(|err| format!("Couldn't parse address: {}", err))?;
            let value = right
                .parse::<u64>()
                .map_err(|err| format!("Couldn't parse value: {}", err))?
                .into();
            Ok(Assignment::Mem { address, value })
        } else {
            Err(format!("Unrecognized operation {}", s))
        }
    }
}

impl FromStr for Mask36 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err("Wrong length of mask".into());
        }
        let mask = s
            .chars()
            .map(|c| match c {
                '0' => Ok(Some(false)),
                '1' => Ok(Some(true)),
                'X' => Ok(None),
                _ => Err(format!("Invalid mask char {}", c)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(mask))
    }
}

impl Default for Num36 {
    fn default() -> Self {
        Self(repeat(false).take(36).collect())
    }
}

impl Default for Mask36 {
    fn default() -> Self {
        Self(repeat(None).take(36).collect())
    }
}

impl From<u64> for Num36 {
    fn from(mut n: u64) -> Self {
        let mut bits = Vec::with_capacity(36);
        while n > 0 {
            bits.push(n & 1 == 1);
            n >>= 1;
        }
        while bits.len() < 36 {
            bits.push(false);
        }
        bits.reverse();
        Num36(bits)
    }
}

impl Into<u64> for Num36 {
    fn into(self) -> u64 {
        let mut rv = 0;
        for bit in self.0 {
            rv <<= 1;
            rv |= if bit { 1 } else { 0 };
        }
        rv
    }
}

impl Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assignment::Mem { address, value } => {
                write!(
                    f,
                    "mem[{}] = {}",
                    address,
                    <Num36 as Into<u64>>::into(value.clone())
                )
            }
            Assignment::Mask { mask } => {
                write!(f, "mask = {}", mask)
            }
        }
    }
}

impl Display for Mask36 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .0
            .iter()
            .map(|b| match b {
                Some(true) => '1',
                Some(false) => '0',
                None => 'X',
            })
            .collect();
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Assignment>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D14-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
                mem[8] = 11
                mem[7] = 101
                mem[8] = 0
            "},
                165,
            )?
            .transformed_case("Solution", include_str!("input"), 13_727_901_897_109)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut memory: HashMap<u64, Num36> = HashMap::new();
        let mut mask: Mask36 = Mask36::default();

        for instruction in input {
            match instruction {
                Assignment::Mem { address, value } => {
                    memory.insert(address, value.with_mask(&mask));
                }
                Assignment::Mask { mask: m } => mask = m,
            }
        }

        Ok(memory
            .into_values()
            .map(|n| {
                let n: u64 = n.into();
                n
            })
            .sum())
    }
}

#[derive(Debug)]
struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Assignment>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D14-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                mask = 000000000000000000000000000000X1001X
                mem[42] = 100
                mask = 00000000000000000000000000000000X0XX
                mem[26] = 1
            "},
                208,
            )?
            .transformed_case("Solution", include_str!("input"), 5_579_916_171_823)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let mut memory: HashMap<u64, Num36> = HashMap::new();
        let mut address_mask: Mask36 = Mask36::default();

        for instruction in input {
            match instruction {
                Assignment::Mem { address, value } => {
                    for masked_address in address_mask.modify_address(address) {
                        memory.insert(masked_address, value.clone());
                    }
                }
                Assignment::Mask { mask: m } => address_mask = m,
            }
        }

        Ok(memory
            .into_values()
            .map(|n| {
                let n: u64 = n.into();
                n
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num36_round_trip() {
        let n1: u64 = 165;
        let n2: Num36 = n1.into();
        let n3: u64 = n2.into();
        assert_eq!(n1, n3);
    }

    #[test]
    fn test_num36() {
        let n1: u64 = 0b0000_1010_1010_1010_1010_1010_1010_1010_1010;
        let n2: Num36 = n1.into();
        assert_eq!(
            n2.0,
            vec![
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true, false, true, false, true, false, true, false, true,
                false, true, false, true, false, true, false, true, false, false, false, false,
            ]
        );
    }

    #[test]
    fn test_assignment_parse() -> Result<(), Box<dyn Error>> {
        let s = "mem[8] = 11";
        let assignment: Assignment = s.parse()?;
        assert_eq!(
            assignment,
            Assignment::Mem {
                address: 8,
                value: Num36(vec![
                    true, true, false, true, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false, false, false, false, false, false, false, false, false,
                    false, false, false,
                ])
            }
        );
        if let Assignment::Mem { value, .. } = &assignment {
            assert_eq!(<Num36 as Into<u64>>::into(value.clone()), 11);
        }
        assert_eq!(format!("{}", assignment), s);
        Ok(())
    }
}
