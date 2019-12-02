use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D02-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, &'static str, u32>::build_set()
            .case("Example", "5 9 2 8\n9 4 7 3\n3 8 6 5\n", 9_u32)
            .case("Solution", include_str!("input"), 214_u32)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let lines: Vec<&str> = input.lines().collect();
        let rows: Vec<Vec<u32>> = lines
            .iter()
            .map(|l| l.split_whitespace().map(|s| s.parse().unwrap()).collect())
            .collect();

        let mut sum = 0;

        'row: for row in rows {
            for (i, first) in row.iter().enumerate() {
                for second in row[(i + 1)..].iter() {
                    let (small, big) = if first < second {
                        (first, second)
                    } else {
                        (second, first)
                    };
                    if big % small == 0 {
                        sum += big / small;
                        continue 'row;
                    }
                }
            }
            panic!(format!("Could not find divisible pair in {:?}", row));
        }

        sum
    }
}
