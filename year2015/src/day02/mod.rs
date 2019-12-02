use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

pub fn parse_input(s: &'static str) -> Vec<(u32, u32, u32)> {
    s.trimmed_lines()
        .map(|l| l.split('x').map(|n| n.parse().unwrap()).collect())
        .map(|mut dimensions: Vec<u32>| {
            assert_eq!(dimensions.len(), 3);
            dimensions.sort();
            (dimensions[0], dimensions[1], dimensions[2])
        })
        .collect()
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2015-D02-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "2x3x4", 58)
            .case("Example 2", "1x1x10", 43)
            .case("Solution", include_str!("input"), 1_588_178)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        parse_input(input)
            .into_iter()
            .map(|(a, b, c)| 3 * a * b + 2 * b * c + 2 * a * c)
            .sum()
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2015-D02-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "2x3x4", 34)
            .case("Example 2", "1x1x10", 14)
            .case("Solution", include_str!("input"), 3_783_758)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        parse_input(input)
            .into_iter()
            .map(|(a, b, c)| {
                let ribbon = 2 * a + 2 * b;
                let volume = a * b * c;
                ribbon + volume
            })
            .sum()
    }
}
