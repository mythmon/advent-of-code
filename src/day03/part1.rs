use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Day03Part1;

impl PuzzleRunner for Day03Part1 {
    type Input = u32;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D03-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", 1, 0)
            .case("Example 2", 12, 3)
            .case("Example 3", 23, 2)
            .case("Example 4", 1_024, 31)
            .case("Solution", 289_326, 419)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut layer = 0;
        let mut layer_max = 0;

        for i in 0.. {
            let odd = i * 2 + 1;
            layer_max = odd * odd;
            if input <= layer_max {
                layer = i;
                break;
            }
        }

        let mut center = layer_max - layer;
        for _ in 0..4 {
            if input >= center - layer {
                let offset = ((center as i32) - (input as i32)).abs() as u32;
                return layer + offset;
            }
            center -= layer * 2;
        }

        unreachable!("should have returned by now");
    }
}
