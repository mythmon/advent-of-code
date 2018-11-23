use std::collections::HashSet;

fn main() {
    let input: &'static str = include_str!("input");
    let input = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{}", puzzle(input));
}

fn puzzle(mut input: Vec<usize>) -> usize {
    if input.len() == 0 {
        return 0;
    }

    let mut seen = HashSet::new();
    seen.insert(input.clone());

    for count in 1.. {
        let mut max_idx = 0;
        let mut max = input[0];

        for (idx, &item) in input.iter().enumerate() {
            if item > max {
                max = item;
                max_idx = idx;
            }
        }

        let remaining = max;
        input[max_idx] = 0;
        for idx in (max_idx + 1)..(max_idx + remaining + 1) {
            let wrapped_idx = idx % input.len();
            input[wrapped_idx] += 1;
        }

        if seen.contains(&input) {
            return count;
        }
        seen.insert(input.clone());
    }

    unreachable!();
}

#[test]
fn test_example() {
    let input = vec![0, 2, 7, 0];
    println!("");
    assert_eq!(puzzle(input), 5);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input");
    let input = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(puzzle(input), 14029);
}
