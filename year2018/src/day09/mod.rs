use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use regex::Regex;
use std::{collections::VecDeque, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D09-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example 1",
                "9 players; last marble is worth 25 points",
                32_u32,
            )
            .case(
                "Example 2",
                "10 players; last marble is worth 1618 points",
                8_317_u32,
            )
            .case(
                "Example 3",
                "13 players; last marble is worth 7999 points",
                146_373_u32,
            )
            .case(
                "Example 4",
                "17 players; last marble is worth 1104 points",
                2_764_u32,
            )
            .case(
                "Example 5",
                "21 players; last marble is worth 6111 points",
                54_718_u32,
            )
            .case(
                "Example 6",
                "30 players; last marble is worth 5807 points",
                37_305_u32,
            )
            .case("Solution", include_str!("input"), 398_502_u32)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let game: GameDescription = input.parse().unwrap();
        assert!(game.last_marble > 1);
        let scores = game.play();
        *scores.iter().max().unwrap()
    }
}

struct GameDescription {
    num_players: u32,
    last_marble: u32,
}

impl GameDescription {
    fn play(&self) -> Vec<u32> {
        let mut marbles: VecDeque<u32> = VecDeque::new();
        marbles.push_back(0);
        let mut scores = Vec::new();
        scores.resize_with(self.num_players as usize, Default::default);

        let turns = (0_usize..self.num_players as usize).cycle();

        for (player, marble) in turns.zip(1..=self.last_marble) {
            // println!("{:?}", marbles);
            if marble % 23 == 0 {
                // something entirely different happens
                scores[player] += marble;
                marbles.rotate_cw_n(7);
                scores[player] += marbles.pop_back().unwrap();
                marbles.rotate_ccw();
            } else {
                // insert the marble
                marbles.rotate_ccw();
                marbles.push_front(marble);
                marbles.rotate_ccw();
            }
        }

        scores
    }
}

impl std::str::FromStr for GameDescription {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number_re = Regex::new(r#"(\d+)"#).unwrap();
        let numbers: Vec<u32> = number_re
            .captures_iter(s)
            .map(|captures| captures.get(0).unwrap().as_str().parse().unwrap())
            .collect();
        assert!(numbers.len() == 2);
        Ok(Self {
            num_players: numbers[0],
            last_marble: numbers[1],
        })
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2018-D09-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input"), 3_352_920_421)
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut game: GameDescription = input.parse().unwrap();
        assert!(game.last_marble > 1);
        game.last_marble *= 100;
        let scores = game.play();
        *scores.iter().max().unwrap()
    }
}

trait Rotate {
    /// ABCD -> DABC
    fn rotate_cw(&mut self);

    /// ABCD -> BCDA
    fn rotate_ccw(&mut self);

    fn rotate_cw_n(&mut self, n: usize) {
        for _ in 0..n {
            self.rotate_cw();
        }
    }

    fn rotate_ccw_n(&mut self, n: usize) {
        for _ in 0..n {
            self.rotate_ccw();
        }
    }
}

impl<T> Rotate for VecDeque<T> {
    fn rotate_cw(&mut self) {
        if !self.is_empty() {
            let tmp = self.pop_back().unwrap();
            self.push_front(tmp);
        }
    }

    fn rotate_ccw(&mut self) {
        if !self.is_empty() {
            let tmp = self.pop_front().unwrap();
            self.push_back(tmp);
        }
    }
}
