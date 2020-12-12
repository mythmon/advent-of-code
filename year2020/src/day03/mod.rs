use advent_lib::{
    cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner},
    helpers::StringAdventExt,
};
use indoc::indoc;
use std::iter::Iterator;

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

fn tree_on_slopes(trees: &[Vec<bool>], dx: usize, dy: usize) -> u64 {
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    while y < trees.len() {
        if trees[y][x] {
            tree_count += 1;
        }
        x = (x + dx) % trees[y].len();
        y += dy;
    }

    tree_count
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<Vec<bool>>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D03-P1".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                    ..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#
                "},
                7,
            )?
            .transformed_case("Solution", include_str!("input"), 276)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        tree_on_slopes(&input, 3, 1)
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<Vec<bool>>;
    type Output = u64;

    fn name(&self) -> String {
        "2020-D03-P2".to_owned()
    }

    fn cases(&self) -> Result<Vec<Box<dyn PuzzleCase>>, Box<dyn std::error::Error>> {
        Ok(GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case(
                "Example",
                indoc! {"
                    ..##.......
                    #...#...#..
                    .#....#..#.
                    ..#.#...#.#
                    .#...##..#.
                    ..#.##.....
                    .#.#.#....#
                    .#........#
                    #.##...#...
                    #...##....#
                    .#..#...#.#
                "},
                336,
            )?
            .transformed_case("Solution", include_str!("input"), 7_812_180_000)?
            .collect())
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .into_iter()
            .map(|(dx, dy)| tree_on_slopes(&input, dx, dy))
            .product()
    }
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trimmed_lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(format!("unexpected char {}", c)),
                })
                .collect()
        })
        .collect()
}
