use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    extremes,
};

#[derive(Debug)]
pub struct Day02Part1;

impl PuzzleRunner for Day02Part1 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D02-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, &'static str, u32>::build_set()
            .case("Example", "5 1 9 5\n7 5 3\n2 4 6 8\n", 18)
            .case("Solution", include_str!("input"), 34_581)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let lines: Vec<&str> = input.lines().collect();
        let rows: Vec<Vec<u32>> = lines
            .iter()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect();

        let mut sum = 0;

        for row in rows {
            let (min, max): (u32, u32) = extremes(row).unwrap();
            sum += max - min;
        }

        sum
    }
}
