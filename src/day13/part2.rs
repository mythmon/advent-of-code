use std::collections::HashSet;
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
    let scanners: Vec<Scanner> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    let search_len = scanners.iter().fold(scanners[0].range, |acc, scanner| {
        lcm(acc, scanner.cycle_length())
    });
    let mut search: Vec<bool> = Vec::with_capacity(search_len);
    search.resize(search_len, true);

    let sieve_values: HashSet<Scanner> = scanners.into_iter().collect();

    for scanner in sieve_values {
        let skip = {
            let cycle = scanner.cycle_length() as i64;
            let depth = scanner.depth as i64;
            let mut skip = cycle - depth;
            while skip < 0 {
                skip += cycle;
            }
            skip as usize
        };
        for slot in search.iter_mut().skip(skip).step_by(scanner.cycle_length()) {
            *slot = false;
        }
    }

    search.iter().enumerate().filter(|v| *v.1).next().unwrap().0
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    // euclid's algorithm
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Scanner {
    depth: usize,
    range: usize,
}

impl Scanner {
    fn new(depth: usize, range: usize) -> Self {
        assert!(range >= 1);
        Self {
            depth: depth,
            range: range,
        }
    }

    #[inline]
    fn cycle_length(&self) -> usize {
        if self.range == 1 {
            1
        } else {
            (self.range - 1) * 2
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "0: 3\n1: 2\n4: 4\n6: 4\n";
        assert_eq!(puzzle(input), 10);
    }

    #[test]
    fn test_correct_answer() {
        let input = get_input();
        assert_eq!(puzzle(input), 3876272);
    }
}
