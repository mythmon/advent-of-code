use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> usize {
    let mut scanners: HashMap<usize, Scanner> = HashMap::new();
    let mut max_depth = 0;
    for line in input.trim().lines() {
        let scanner: Scanner = line.parse().unwrap();
        max_depth = cmp::max(scanner.depth, max_depth);
        scanners.insert(scanner.depth, scanner);
    }

    let mut severity = 0;

    for packet_position in 0..max_depth + 1 {
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

#[derive(Debug)]
struct Scanner {
    depth: usize,
    range: usize,
    position: usize,
    going_up: bool,
}

impl Scanner {
    fn new(depth: usize, range: usize) -> Self {
        Self {
            depth: depth,
            range: range,
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

    fn severity(&self) -> usize {
        self.depth * self.range
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<usize> = input.split(": ").filter_map(|p| p.parse().ok()).collect();
        if parts.len() != 2 {
            Err(())
        } else {
            Ok(Self::new(parts[0], parts[1]))
        }
    }
}

#[test]
fn test_example() {
    let input = "0: 3\n1: 2\n4: 4\n6: 4\n";
    assert_eq!(puzzle(input), 24);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 2688);
}
