use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::{indoc, indoc_impl};
use std::{collections::HashMap, iter::Iterator, str::FromStr};

mod parser;

#[derive(Debug)]
pub struct Day03Part1;

impl PuzzleRunner for Day03Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2018-D03-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc!(
                    "
                #1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2
                "
                ),
                4usize,
            )
            .case("Solution", include_str!("input"), 109_143usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let claims: Vec<Claim> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();
        let mut taken = HashMap::new();
        // TODO go more than one-at-a-time?
        for claim in claims {
            for x in claim.x..(claim.x + claim.w) {
                for y in claim.y..(claim.y + claim.h) {
                    *taken.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        taken.values().filter(|c| **c > 1).count()
    }
}

#[derive(Debug)]
pub struct Day03Part2;

impl PuzzleRunner for Day03Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D03-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc!(
                    "
                #1 @ 1,3: 4x4
                #2 @ 3,1: 4x4
                #3 @ 5,5: 2x2
                "
                ),
                3u32,
            )
            .case("Solution", include_str!("input"), 506u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let claims: Vec<Claim> = input.trimmed_lines().map(|l| l.parse().unwrap()).collect();
        let mut taken = HashMap::new();
        // TODO can this be done without iterating twice?
        for claim in claims.iter() {
            for x in claim.x..(claim.x + claim.w) {
                for y in claim.y..(claim.y + claim.h) {
                    *taken.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        for claim in claims {
            let mut collided = false;
            for x in claim.x..(claim.x + claim.w) {
                for y in claim.y..(claim.y + claim.h) {
                    if taken[&(x, y)] != 1 {
                        collided = true;
                        break;
                    }
                }
            }
            if !collided {
                return claim.id;
            }
        }
        panic!("no answer found");
    }
}

// TODO add area iterator
pub struct Claim {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl FromStr for Claim {
    // TODO better error handling
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::ClaimParser::new().parse(s).map_err(|_| ())
    }
}
