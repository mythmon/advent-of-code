use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day17Part1;

impl PuzzleRunner for Day17Part1 {
    type Input = usize;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D17-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.trim().parse().unwrap())
            .case("Example", 3, 638)
            .transformed_case("Solution", include_str!("input"), 1_244)
            .collect()
    }

    fn run_puzzle(step: Self::Input) -> Self::Output {
        let mut buffer = Vec::with_capacity(2018);
        buffer.push(0);
        let mut pos = 0;

        let max = 2018;

        for i in 1..max {
            pos = (pos + step) % buffer.len();
            buffer.insert(pos + 1, i);
            pos += 1;
        }

        buffer[(pos + 1) % buffer.len()]
    }
}
