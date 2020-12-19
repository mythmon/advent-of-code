use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use lalrpop_util::lalrpop_mod;

#[cfg(windows)]
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    parser,
    "\\day18\\parser.rs"
);
#[cfg(unix)]
lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    parser,
    "/day18/parser.rs"
);

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D18-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "1 + 2 * 3 + 4 * 5 + 6", 71)
            .case("Example 2", "1 + (2 * 3) + (4 * (5 + 6))", 51)
            .case("Example 3", "2 * 3 + (4 * 5)", 26)
            .case("Example 4", "5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)
            .case(
                "Example 5",
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                12240,
            )
            .case(
                "Example 6",
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                13632,
            )
            .case("Solution", include_str!("input"), 98_621_258_158_412)
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let parser = parser::P1ExprParser::new();
        Ok(input
            .trimmed_lines()
            .map(|l| parser.parse(l))
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|err| format!("Error: {}", err))?
            .into_iter()
            .sum())
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D18-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", "1 + 2 * 3 + 4 * 5 + 6", 231)
            .case("Example 2", "1 + (2 * 3) + (4 * (5 + 6))", 51)
            .case("Example 3", "2 * 3 + (4 * 5)", 46)
            .case("Example 4", "5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)
            .case(
                "Example 5",
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
                669_060,
            )
            .case(
                "Example 6",
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
                23340,
            )
            .case("Solution", include_str!("input"), 241_216_538_527_890)
            .collect())
    }

    fn try_run_puzzle(input: Self::Input) -> Result<Self::Output, Self::Error> {
        let parser = parser::P2ExprParser::new();
        Ok(input
            .trimmed_lines()
            .map(|l| parser.parse(l))
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|err| format!("Error: {}", err))?
            .into_iter()
            .sum())
    }
}
