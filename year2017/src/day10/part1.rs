use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = (usize, Vec<usize>);
    type Output = usize;

    fn name(&self) -> String {
        "2017-D10-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", (5, vec![3, 4, 1, 5]), 12_usize)
            .case(
                "Solution",
                (
                    256,
                    include_str!("input")
                        .split(',')
                        .map(|p| p.trim().parse().unwrap())
                        .collect(),
                ),
                37_230_usize,
            )
            .collect()
    }

    fn run_puzzle((length, instructions): Self::Input) -> Self::Output {
        let k = knot(length, &instructions);
        k[0] * k[1]
    }
}

fn knot(length: usize, instructions: &[usize]) -> Vec<usize> {
    let mut items: Vec<usize> = (0..length).collect();
    let mut position = 0;

    for (skip_size, instr) in instructions.iter().enumerate() {
        let mut section: Vec<usize> = if position + instr < length {
            let range = position..(position + instr);
            Vec::from(&items[range])
        } else {
            let mut part1 = Vec::from(&items[position..]);
            let mut part2 = Vec::from(&items[..(position + instr) % length]);
            part1.append(&mut part2);
            part1
        };
        section.reverse();

        for (i, v) in section.into_iter().enumerate() {
            items[(i + position) % length] = v;
        }
        position = (position + instr + skip_size) % length;
    }

    items
}

#[test]
fn test_knot_example() {
    let instructions = vec![3, 4, 1, 5];
    assert_eq!(knot(5, &instructions), vec![3, 4, 2, 1, 0]);
}
