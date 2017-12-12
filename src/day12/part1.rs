use std::collections::HashSet;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> usize {
    let connections: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" <-> ").collect();
            assert_eq!(parts.len(), 2);
            parts[1]
                .split(",")
                .map(|p| p.trim().parse().unwrap())
                .collect()
        })
        .collect();

    let mut zero_group = HashSet::new();
    zero_group.insert(0);
    let mut changed;
    loop {
        changed = false;
        for (idx, connections) in connections.iter().enumerate() {
            if zero_group.contains(&(idx as u32)) {
                continue;
            }
            for c in connections {
                if zero_group.contains(c) {
                    changed = true;
                    zero_group.insert(idx as u32);
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }

    zero_group.len()
}

#[test]
fn test_example() {
    let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";
    assert_eq!(puzzle(input), 6);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 288);
}
