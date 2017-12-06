use std::collections::HashMap;

fn main() {
    let input: &'static str = include_str!("input.txt");
    let input = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    println!("{}", puzzle(input));
}

fn puzzle(mut input: Vec<usize>) -> i32 {
    if input.len() == 0 {
        return 0;
    }

    let mut seen_at = HashMap::new();
    seen_at.insert(input.clone(), 0);

    for count in 1.. {
        let mut max_idx = 0;
        let mut max = input[0];

        for (idx, &item) in input.iter().enumerate() {
            if item > max {
                max = item;
                max_idx = idx;
            }
        }

        let mut remaining = max;
        input[max_idx] = 0;
        for idx in (max_idx + 1)..(max_idx + remaining + 1) {
            let wrapped_idx = idx % input.len();
            input[wrapped_idx] += 1;
        }

        if seen_at.contains_key(&input) {
            return count - seen_at.get(&input).unwrap();
        }
        seen_at.insert(input.clone(), count);
    }

    unreachable!();
}

#[test]
fn test_example() {
    let input = vec![0, 2, 7, 0];
    assert_eq!(puzzle(input), 4);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input.txt");
    let input = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    assert_eq!(puzzle(input), 2765);
}
