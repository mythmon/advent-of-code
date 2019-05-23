use crate::cases::{GenericPuzzleCase, PuzzleCase, PuzzleRunner};
use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Part1;

impl PuzzleRunner for Part1 {
    type Input = &'static str;
    type Output = usize;

    fn name(&self) -> String {
        "2017-D13-P1".to_owned()
    }

    fn cases(&self) -> Vec<Box<dyn PuzzleCase>> {
        GenericPuzzleCase::<Self, _, _>::build_set()
            .case("Example", "0: 3\n1: 2\n4: 4\n6: 4\n", 24_usize)
            .case("Solution", include_str!("input"), 2_688_usize)
            .collect()
    }

    fn run_puzzle(input: Self::Input) -> Self::Output {
        let mut scanners: HashMap<usize, Scanner> = HashMap::new();
        let mut max_depth = 0;
        for line in input.trim().lines() {
            let scanner: Scanner = line
                .parse()
                .unwrap_or_else(|_| panic!("Could not parse line: {:?}", &line));
            max_depth = cmp::max(scanner.depth, max_depth);
            scanners.insert(scanner.depth, scanner);
        }

        let mut severity = 0;

        for packet_position in 0..=max_depth {
            if let Some(scanner) = scanners.get(&packet_position) {
                if scanner.position == 0 {
                    severity += scanner.severity();
                }
            }
            for scanner in scanners.values_mut() {
                scanner.tick();
            }
        }

        severity
    }
}

#[derive(Debug)]
struct Scanner {
    depth: usize,
    range: usize,
    position: usize,
    going_up: bool,
}

impl Scanner {
    const fn new(depth: usize, range: usize) -> Self {
        Self {
            depth,
            range,
            position: 0,
            going_up: true,
        }
    }

    fn tick(&mut self) {
        if self.going_up {
            self.position += 1;
            if self.position == self.range - 1 {
                self.going_up = false;
            }
        } else {
            self.position -= 1;
            if self.position == 0 {
                self.going_up = true;
            }
        }
    }

    const fn severity(&self) -> usize {
        self.depth * self.range
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = input.split(": ").filter_map(|p| p.parse().ok()).collect();
        if parts.len() == 2 {
            Ok(Self::new(parts[0], parts[1]))
        } else {
            Err(format!(
                "Wrong number of parts, expected 2, got {}",
                parts.len()
            ))
        }
    }
}
