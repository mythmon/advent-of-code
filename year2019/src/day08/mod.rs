use advent_lib::cases::{GenericPuzzleCase, Puzzle, PuzzleCase, PuzzleRunner};
use std::{collections::HashMap, iter::Iterator};

pub fn get_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![Box::new(Part1), Box::new(Part2)]
}

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<char>;
    type Output = usize;

    fn name(&self) -> String {
        "2019-D08-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), 1088)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let layer_size = 25 * 6;
        let layer_with_least_zeroes: &[char] = input
            .chunks(layer_size)
            .map(|layer| (layer, layer.iter().filter(|c| **c == '0').count()))
            .min_by_key(|(_, num_zeroes)| *num_zeroes)
            .unwrap()
            .0;
        let char_counts: HashMap<char, usize> = layer_with_least_zeroes.iter().fold(
            HashMap::new(),
            |mut acc: HashMap<char, usize>, c: &char| {
                *acc.entry(*c).or_insert(0) += 1;
                acc
            },
        );
        char_counts[&'1'] * char_counts[&'2']
    }
}

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = Vec<char>;
    type Output = ();

    fn name(&self) -> String {
        "2019-D08-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(parse_input)
            .transformed_case("Solution", include_str!("input"), ())
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let width = 25;
        let height = 6;
        let layers: Vec<&[char]> = input.chunks(width * height).collect();
        for y in 0..height {
            for x in 0..width {
                let pixel_offset = x + y * width;
                let pixel = layers
                    .iter()
                    .map(|l| l[pixel_offset])
                    .find(|p| *p != '2')
                    .unwrap_or('0');
                match pixel {
                    '0' => print!("  "),
                    '1' => print!("██"),
                    _ => panic!("unexpected, non-transparent pixel {}", pixel),
                }
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}
