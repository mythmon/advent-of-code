use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::{
    collections::{HashMap, VecDeque},
    iter::Iterator,
    str::FromStr,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Rule>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D07-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                light red bags contain 1 bright white bag, 2 muted yellow bags.
                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                bright white bags contain 1 shiny gold bag.
                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                faded blue bags contain no other bags.
                dotted black bags contain no other bags.
            "},
                4,
            )?
            .transformed_case("Solution", include_str!("input"), 302)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut todo = input.iter().collect::<VecDeque<_>>();
        let mut can_contain_gold: HashMap<String, bool> = HashMap::new();

        // Assumption
        can_contain_gold.insert("shiny gold".to_string(), false);

        let mut iterations_left = todo.len().pow(3);
        while !todo.is_empty() {
            iterations_left -= 1;
            if iterations_left == 0 {
                panic!("Too much repetition");
            }
            let to_check = todo.pop_front().unwrap();

            // See if we've already checked this color
            if can_contain_gold.contains_key(&to_check.outer_color) {
                continue;
            }

            // Empty bags can't contain anything
            if to_check.contains.is_empty() {
                can_contain_gold.insert(to_check.outer_color.clone(), false);
                continue;
            }

            // If the bag directly contains shiny gold, or contains a bag that
            // can (possibly indirectly) contain shiny gold, mark it as being
            // able to contain gold.
            if to_check.contains.iter().any(|(_, inner_color)| {
                inner_color == "shiny gold"
                    || matches!(can_contain_gold.get(inner_color), Some(true))
            }) {
                can_contain_gold.insert(to_check.outer_color.clone(), true);
                continue;
            }

            // If anything is unknown, there is still hope, put it back on the pile.
            if to_check
                .contains
                .iter()
                .any(|(_, inner_color)| can_contain_gold.get(inner_color) == None)
            {
                todo.push_back(to_check);
                continue;
            }

            // Otherwise it can't
            can_contain_gold.insert(to_check.outer_color.clone(), false);
        }

        can_contain_gold.values().filter(|b| **b).count()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Rule>;
    type Output = usize;

    fn name(&self) -> String {
        "2020-D07-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case(
                "Example 1",
                indoc! {"
                light red bags contain 1 bright white bag, 2 muted yellow bags.
                dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                bright white bags contain 1 shiny gold bag.
                muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                faded blue bags contain no other bags.
                dotted black bags contain no other bags.
            "},
                32,
            )?
            .transformed_case(
                "Example 2",
                indoc! {"
                shiny gold bags contain 2 dark red bags.
                dark red bags contain 2 dark orange bags.
                dark orange bags contain 2 dark yellow bags.
                dark yellow bags contain 2 dark green bags.
                dark green bags contain 2 dark blue bags.
                dark blue bags contain 2 dark violet bags.
                dark violet bags contain no other bags.
            "},
                126,
            )?
            .transformed_case("Solution", include_str!("input"), 4_165)?
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let map: HashMap<String, Vec<(usize, String)>> = input
            .into_iter()
            .map(|rule| (rule.outer_color, rule.contains))
            .collect();

        Ok(count_bags_in(&map, &("shiny gold".to_string()))? - 1)
    }
}

fn count_bags_in(
    source: &HashMap<String, Vec<(usize, String)>>,
    target: &String,
) -> Result<usize, String> {
    let bag_info = source
        .get(target)
        .ok_or(format!("Target `{}` not in source", target))?;

    Ok(1_usize
        + (bag_info
            .iter()
            .map(|(count, inner_color)| Ok(count * count_bags_in(source, inner_color)?))
            .collect::<Result<Vec<usize>, String>>()?
            .iter()
            .sum::<usize>()))
}

#[derive(Clone, Debug)]
pub struct Rule {
    outer_color: String,
    contains: Vec<(usize, String)>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (outer_color, rest) = s
            .split_once(" bags contain ")
            .ok_or("Syntax error, did not match first split")?;
        let contains = rest
            .split(",")
            .flat_map(|s| {
                s.split(",")
                    .map(|p| p.trim())
                    .flat_map(|p| {
                        let p = p.trim_end_matches('.');
                        if p == "no other bags" {
                            vec![]
                        } else {
                            if let Some((count, description)) = p.split_once(' ') {
                                if let Ok(count) = count.parse() {
                                    let description = description
                                        .trim_end_matches(".")
                                        .trim_end_matches("s")
                                        .trim_end_matches("bag")
                                        .trim();
                                    vec![Ok((count, description.to_string()))]
                                } else {
                                    vec![Err(format!("Syntax error: Bad digit in `{}`", p))]
                                }
                            } else {
                                vec![Err(format!("Syntax error: Bad format in `{}`", p))]
                            }
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Result<Vec<(_, _)>, _>>()?;
        Ok(Self {
            outer_color: outer_color.to_string(),
            contains,
        })
    }
}

fn parse_input(input: &str) -> Vec<Rule> {
    input
        .trimmed_lines()
        .map(|line| line.parse().unwrap())
        .collect()
}
