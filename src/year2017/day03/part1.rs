use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = u32;
    type Output = u32;

    fn name(&self) -> String {
        "2017-D03-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example 1", 1_u32, 0_u32)
            .case("Example 2", 12_u32, 3_u32)
            .case("Example 3", 23_u32, 2_u32)
            .case("Example 4", 1_024_u32, 31_u32)
            .case("Solution", 289_326_u32, 419_u32)
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
