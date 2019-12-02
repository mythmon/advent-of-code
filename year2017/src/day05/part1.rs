use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = Vec<i32>;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D05-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .add_transform(|s| s.lines().map(|l| l.parse().unwrap()).collect())
            .case("Example", vec![0, 3, 0, 1, -3], 5_u32)
            .transformed_case("Solution", include_str!("input"), 388_611_u32)
            .collect()
    }

    fn run_puzzle(mut input: Self::Input) -> Self::Output {
        let mut steps: u32 = 0;
        let mut pc: i32 = 0;

        let bounds = 0..(input.len() as i32);

        while bounds.contains(&pc) {
            steps += 1;
            let next = pc + input[pc as usize];
            input[pc as usize] += 1;
            pc = next;
        }

        steps
    }
}
