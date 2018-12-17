use crate::{
    cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner},
    year2017::day18::{Instr, Machine},
};
use indoc::{indoc, indoc_impl};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Day18Part2;

impl PuzzleRunner for Day18Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D18-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case(
                "Example",
                indoc!(
                    "
                    snd 1
                    snd 2
                    snd p
                    rcv a
                    rcv b
                    rcv c
                    rcv d"
                ),
                3_usize,
            )
            .case("Solution", include_str!("input"), 5_969_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let instructions: Vec<Instr> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
        let mut m0 = Machine::new(0, instructions.clone());
        let mut m1 = Machine::new(1, instructions);

        let mut io = VecDeque::new();
        let mut to_m0_queue = m1.run_until_blocked(&mut io);
        let mut m1_send_count = to_m0_queue.len();
        let mut to_m1_queue = m0.run_until_blocked(&mut io);

        loop {
            io = m1.run_until_blocked(&mut to_m1_queue);
            if io.is_empty() {
                // m0 is blocked on RCV, but m1 didn't SND anything.
                break;
            }
            m1_send_count += io.len();
            to_m0_queue.append(&mut io);

            io = m0.run_until_blocked(&mut to_m0_queue);
            if io.is_empty() {
                // m1 is blocked on RCV, but m0 didn't SND anything.
                break;
            }
            to_m1_queue.append(&mut io);
        }

        m1_send_count
    }
}
