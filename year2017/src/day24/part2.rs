use advent_lib::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub struct Part2;

impl PuzzleRunner for Part2 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D24-P2".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Solution", include_str!("input").trim(), 1_824_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut parts: VecDeque<Part> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut bridges = vec![];
        make_bridges(&mut bridges, &mut parts, &mut Bridge(vec![]));
        let bridge = bridges.iter().max_by_key(|b| (b.0.len(), b.strength()));
        bridge.unwrap().strength()
    }
}

fn make_bridges(bridges: &mut Vec<Bridge>, left: &mut VecDeque<Part>, partial_bridge: &mut Bridge) {
    // backtrack algorithm from wikipedia
    // procedure bt(c)
    //     if reject(P,c) then return
    //     if accept(P,c) then output(P,c)
    //     s ← first(P,c)
    //     while s ≠ Λ do
    //         bt(s)
    //         s ← next(P,s)

    // bridges don't become invalid, unless maybe they couldn't
    // possibly be strong enough? This is a potential optimization.

    // if accept
    if left.is_empty() {
        bridges.push(partial_bridge.clone());
    }

    for _ in 0..left.len() {
        let part = left.pop_back().unwrap();
        let new_bridge = partial_bridge.add(part);
        if let Some(mut new_bridge) = new_bridge {
            bridges.push(new_bridge.clone());
            make_bridges(bridges, left, &mut new_bridge);
        }
        left.push_front(part);
    }
}

#[derive(Debug, Clone)]
struct Bridge(Vec<Part>);

impl Bridge {
    fn add(&self, mut p: Part) -> Option<Self> {
        let mut new_bridge = self.clone();
        let m = self.next_match();
        if m == p.0 {
            new_bridge.0.push(p);
            Some(new_bridge)
        } else if m == p.1 {
            p.flip();
            new_bridge.0.push(p);
            Some(new_bridge)
        } else {
            None
        }
    }

    fn strength(&self) -> usize {
        self.0.iter().map(|p| p.0 + p.1).sum()
    }

    fn next_match(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[self.0.len() - 1].1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part(usize, usize);

impl Part {
    fn flip(&mut self) {
        std::mem::swap(&mut self.0, &mut self.1);
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = input.split('/').map(|p| p.parse().unwrap()).collect();
        assert_eq!(parts.len(), 2);
        Ok(Self(parts[0], parts[1]))
    }
}

#[test]
fn test_flip() {
    let mut p = Part(1, 2);
    p.flip();
    assert_eq!(p.0, 2);
    assert_eq!(p.1, 1);
}
