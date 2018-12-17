use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::iter::Iterator;

#[derive(Debug)]
pub struct Day15Part2;

impl PuzzleRunner for Day15Part2 {
    type Input = (&'static str, usize);
    type Output = usize;

    fn name(&self) -> String {
        "2017-D15-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example short",
                (
                    "Generator A starts with 65\nGenerator B starts with 8921\n",
                    1056,
                ),
                1_usize,
            )
            .case(
                "Example long",
                (
                    "Generator A starts with 65\nGenerator B starts with 8921\n",
                    5_000_000,
                ),
                309_usize,
            )
            .case("Solution", (include_str!("input"), 5_000_000), 336_usize)
            .collect()
    }

    fn run_puzzle((input, iterations): Self::Input) -> Self::Output {
        let initial_values: Vec<u64> = input
            .lines()
            .filter_map(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                assert_eq!(parts.len(), 5);
                parts[4].parse().ok()
            })
            .collect();

        assert_eq!(initial_values.len(), 2);

        let generator_a = Generator::with_multiple_of(initial_values[0], 16807, 4);
        let generator_b = Generator::with_multiple_of(initial_values[1], 48271, 8);

        generator_a
            .zip(generator_b)
            .take(iterations)
            .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
            .count()
    }
}

struct Generator {
    factor: u64,
    divisor: u64,
    last_value: u64,
    multiple_of: u64,
}

impl Generator {
    fn with_multiple_of(initial_value: u64, factor: u64, multiple_of: u64) -> Self {
        Self {
            factor,
            divisor: 2_147_483_647,
            last_value: initial_value,
            multiple_of,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            let rv = self.last_value * self.factor % self.divisor;
            self.last_value = rv;
            if rv % self.multiple_of == 0 {
                return Some(rv);
            }
        }
    }
}

#[test]
fn test_example_a() {
    let g = Generator::with_multiple_of(65, 16807, 4);
    let values: Vec<u64> = g.take(5).collect();
    assert_eq!(
        values,
        vec![
            1_352_636_452,
            1_992_081_072,
            530_830_436,
            1_980_017_072,
            740_335_192
        ]
    );
}

#[test]
fn test_example_b() {
    let g = Generator::with_multiple_of(8921, 48271, 8);
    let values: Vec<u64> = g.take(5).collect();
    assert_eq!(
        values,
        vec![
            1_233_683_848,
            862_516_352,
            1_159_784_568,
            1_616_057_672,
            412_269_392
        ]
    );
}
