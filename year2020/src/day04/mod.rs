use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use indoc::indoc;
use regex::Regex;
use std::{collections::HashMap, error::Error, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<HashMap<String, String>>;
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
        let required: Vec<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        input
            .iter()
            .filter(|passport| required.iter().all(|key| passport.contains_key(key)))
            .count()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<HashMap<String, String>>;
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
        let required: Vec<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let color_re = Regex::new(r"^#[0-9a-f]{6}$")?;
        let pid_re = Regex::new(r"^[0-9]{9}$")?;

        Ok(input
            .iter()
            // hash all required keys
            .filter(|passport| required.iter().all(|key| passport.contains_key(key)))
            // keys are valid
            .map(|passport| {
                Ok(
                    (1920..=2002).contains(&passport["byr"].parse::<i32>()?)
                    && (2010..=2020).contains(&passport["iyr"].parse::<i32>()?)
                    && (2020..=2030).contains(&passport["eyr"].parse::<i32>()?)
                    && (
                        if passport["hgt"].ends_with("cm") {
                            (150..=193).contains(&passport["hgt"].trim_end_matches("cm").parse::<i32>()?)
                        } else if passport["hgt"].ends_with("in") {
                            (59..=76).contains(&passport["hgt"].trim_end_matches("in").parse::<i32>()?)
                        } else {
                            false
                        }
                    )
                    && color_re.is_match(&passport["hcl"])
                    && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"].as_str())
                    && pid_re.is_match(&passport["pid"])
                )
            })
            .filter_map(|result: Result<bool, Box<dyn Error>>| result.ok())
            .filter(|result| *result)
            .count())
    }
}

fn parse_input(input: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    input
        .trim()
        .split("\n\n")
        .map(|line| {
            // split out parts
            line.split_whitespace()
                .map(|part| {
                    // split each part into pairs
                    if let &[a, b] = part.split(':').collect::<Vec<_>>().as_slice() {
                        // and group them into tuples
                        Ok((a.to_string(), b.to_string()))
                    } else {
                        Err("Colon pair not correct".into())
                    }
                })
                // to build into a hashmap
                .collect()
        })
        // collect a vec of hashmaps
        .collect()
}
