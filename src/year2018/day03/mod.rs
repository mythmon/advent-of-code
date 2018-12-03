#![allow(clippy::many_single_char_names)] // TODO use better names for claim fields

use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::{indoc, indoc_impl};
use std::{collections::HashMap, iter::Iterator, str::FromStr};

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
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl FromStr for Claim {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let id = parts[0].trim_start_matches('#').parse()?;
        let coords: Vec<_> = parts[2].trim_end_matches(':').split(',').collect();
        let x = coords[0].parse()?;
        let y = coords[1].parse()?;
        let span: Vec<_> = parts[3].split('x').collect();
        let w = span[0].parse()?;
        let h = span[1].parse()?;
        Ok(Self { id, x, y, w, h })
    }
}
