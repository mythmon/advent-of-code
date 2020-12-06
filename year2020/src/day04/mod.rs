use advent_lib::{helpers::StringAdventExt, cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner}};
use indoc::indoc;
use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    iter::Iterator,
    lazy::Lazy,
    ops::RangeBounds,
    str::FromStr,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Passport>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D04-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Example 1",
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm",
                1,
            )?
            .transformed_case(
                "Example 2",
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929",
                0,
            )?
            .transformed_case(
                "Example 3",
                "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm",
                1,
            )?
            .transformed_case(
                "Example 4",
                "hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
                0,
            )?
            .transformed_case("Solution", include_str!("input"), 213)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|passport| passport.required_fields_present())
            .count()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Passport>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D04-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_try_transform(parse_input)
            .transformed_case(
                "Invalid Examples",
                indoc! {"
                    eyr:1972 cid:100
                    hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

                    iyr:2019
                    hcl:#602927 eyr:1967 hgt:170cm
                    ecl:grn pid:012533040 byr:1946

                    hcl:dab227 iyr:2012
                    ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

                    hgt:59cm ecl:zzz
                    eyr:2038 hcl:74454a iyr:2023
                    pid:3556412378 byr:2007
                "},
                0,
            )?
            .transformed_case(
                "Valid Examples",
                indoc! {"
                    pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
                    hcl:#623a2f

                    eyr:2029 ecl:blu cid:129 byr:1989
                    iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

                    hcl:#888785
                    hgt:164cm byr:2001 iyr:2015 cid:88
                    pid:545766238 ecl:hzl
                    eyr:2022

                    iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
                "},
                4,
            )?
            .transformed_case("Solution", include_str!("input"), 147)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        Ok(input.iter().filter(|passport| passport.is_valid()).count())
    }
}

fn parse_input(input: &str) -> Result<Vec<Passport>, Box<dyn Error>> {
    input
        .paragraphs()
        .map(|p| p.parse())
        .collect::<Result<Vec<_>, PassportError>>()
        .map_err(|err| err.into())
}

#[derive(Clone, Debug, Default)]
pub struct Passport {
    birth_year: PassportField<u16>,
    issue_year: PassportField<u16>,
    expiry_year: PassportField<u16>,
    height: PassportField<Height>,
    hair_color: PassportField<Color>,
    eye_color: PassportField<EyeColor>,
    passport_id: PassportField<String>,
    country_id: PassportField<String>,
}

impl Passport {
    fn from_data(data: &HashMap<String, String>) -> Result<Self, PassportError> {
        let mut passport = Self::default();
        for (key, value) in data {
            passport.set_from_str(key, value)?;
        }
        Ok(passport)
    }

    fn set_from_str(&mut self, key: &str, value: &str) -> Result<(), PassportError> {
        // let required: Vec<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        match key {
            "byr" => self.birth_year.ranged_set(Some(value), 1920..=2002),
            "iyr" => self.issue_year.ranged_set(Some(value), 2010..=2020),
            "eyr" => self.expiry_year.ranged_set(Some(value), 2020..=2030),
            "hcl" => self.hair_color.set(Some(value)),
            "ecl" => self.eye_color.set(Some(value)),

            "hgt" => self
                .height
                .validated_set(Some(value), |height| match height {
                    Height::Inches(h) => (59..=76).contains(h),
                    Height::Centimeters(h) => (150..=193).contains(h),
                }),

            "pid" => self
                .passport_id
                .validated_set(Some(value), |s| s.len() == 9),

            "cid" => self.country_id.validated_set(Some(value), |s| s.len() == 9),

            _ => return Err(PassportError::UnknownKey(key.to_string())),
        }
        Ok(())
    }

    pub fn required_fields_present(&self) -> bool {
        [
            self.birth_year != PassportField::Missing,
            self.issue_year != PassportField::Missing,
            self.expiry_year != PassportField::Missing,
            self.height != PassportField::Missing,
            self.hair_color != PassportField::Missing,
            self.eye_color != PassportField::Missing,
            self.passport_id != PassportField::Missing,
            // country id is not required
        ]
        .iter()
        .all(|p| *p)
    }

    pub fn is_valid(&self) -> bool {
        [
            matches!(self.birth_year, PassportField::Valid(_)),
            matches!(self.issue_year, PassportField::Valid(_)),
            matches!(self.expiry_year, PassportField::Valid(_)),
            matches!(self.height, PassportField::Valid(_)),
            matches!(self.hair_color, PassportField::Valid(_)),
            matches!(self.eye_color, PassportField::Valid(_)),
            matches!(self.passport_id, PassportField::Valid(_)),
            // country id is not required or validated
        ]
        .iter()
        .all(|p| *p)
    }
}

impl FromStr for Passport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split_whitespace()
                .map(|part| {
                    // split each part into pairs
                    part.split_once(':')
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .ok_or(PassportError::MissingColon)
                })
                // to build into a hashmap
                .collect::<Result<HashMap<String, String>, _>>()?;
        Passport::from_data(&data)
    }
}

#[derive(Clone, Debug)]
pub enum PassportError {
    MissingColon,
    UnknownKey(String),
    InvalidEyeColor(String),
    NumberSyntax(std::num::ParseIntError),
    InvalidUnit(String),
    InvalidColor(String),
}

impl Display for PassportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PassportError::MissingColon => write!(f, "Missing colon"),
            PassportError::UnknownKey(key) => write!(f, "Unknown key {}", key),
            PassportError::InvalidColor(s) => write!(f, "Invalid color {}", s),
            PassportError::InvalidEyeColor(s) => write!(f, "Invalid eye color {}", s),
            PassportError::InvalidUnit(u) => write!(f, "Invalid unit {}", u),
            PassportError::NumberSyntax(err) => write!(f, "Could not parse number: {}", err),
        }
    }
}

impl Error for PassportError {}

#[derive(Clone, Debug, PartialEq)]
pub enum PassportField<T> {
    Valid(T),
    Invalid(String),
    Missing,
}

impl<T: FromStr> PassportField<T> {
    fn set(&mut self, value: Option<&str>) {
        self.validated_set(value, |_| true)
    }

    fn ranged_set<R>(&mut self, value: Option<&str>, range: R)
    where
        R: RangeBounds<T>,
        T: PartialOrd,
    {
        self.validated_set(value, |v| range.contains(v))
    }

    fn validated_set<F>(&mut self, value: Option<&str>, is_valid: F)
    where
        F: Fn(&T) -> bool,
    {
        *self = if let Some(s) = value {
            match s.parse() {
                Ok(parsed) if is_valid(&parsed) => Self::Valid(parsed),
                _ => Self::Invalid(s.to_string()),
            }
        } else {
            Self::Missing
        }
    }
}

impl<T> Default for PassportField<T> {
    fn default() -> Self {
        Self::Missing
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Height {
    Inches(u8),
    Centimeters(u8),
}

impl FromStr for Height {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, unit) = s.split_at(s.len() - 2);
        let num = num
            .parse()
            .map_err(|err| PassportError::NumberSyntax(err))?;
        Ok(match unit {
            "cm" => Self::Centimeters(num),
            "in" => Self::Inches(num),
            _ => return Err(PassportError::InvalidUnit(unit.to_string())),
        })
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

impl FromStr for EyeColor {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "amb" => Self::Amber,
            "blu" => Self::Blue,
            "brn" => Self::Brown,
            "gry" => Self::Grey,
            "grn" => Self::Green,
            "hzl" => Self::Hazel,
            "oth" => Self::Other,
            _ => return Err(PassportError::InvalidEyeColor(s.to_string())),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Color(String);

const COLOR_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^#[0-9a-f]{6}$").unwrap());

impl FromStr for Color {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        if COLOR_RE.is_match(&s) {
            Ok(Self(s))
        } else {
            Err(PassportError::InvalidColor(s))
        }
    }
}
