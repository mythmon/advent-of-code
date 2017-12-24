use std::str::FromStr;
use std::collections::VecDeque;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input.trim()
}

fn puzzle(input: &str) -> usize {
    let mut parts: VecDeque<Part> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut bridges = vec![];
    make_bridges(&mut bridges, &mut parts, Bridge(vec![]));
    let bridge = bridges.iter().max_by_key(|b| (b.0.len(), b.strength()));
    bridge.unwrap().strength()
}

fn make_bridges(bridges: &mut Vec<Bridge>, left: &mut VecDeque<Part>, partial_bridge: Bridge) {
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
        if let Some(new_bridge) = new_bridge {
            bridges.push(new_bridge.clone());
            make_bridges(bridges, left, new_bridge);
        }
        left.push_front(part);
    }
}

#[derive(Debug, Clone)]
struct Bridge(Vec<Part>);

impl Bridge {
    fn add(&self, mut p: Part) -> Option<Bridge> {
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
        if self.0.len() == 0 {
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
        let t = self.0;
        self.0 = self.1;
        self.1 = t;
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = input.split("/").map(|p| p.parse().unwrap()).collect();
        assert_eq!(parts.len(), 2);
        Ok(Part(parts[0], parts[1]))
    }
}

#[test]
fn test_example() {
    let input = "0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10\n";
    assert_eq!(puzzle(input), 31);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    // assert_eq!(puzzle(input), 42);
}

#[test]
fn test_flip() {
    let mut p = Part(1, 2);
    p.flip();
    assert_eq!(p.0, 2);
    assert_eq!(p.1, 1);
}
