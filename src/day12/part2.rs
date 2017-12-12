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
    let mut connections: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" <-> ").collect();
            assert_eq!(parts.len(), 2);
            parts[1]
                .split(",")
                .map(|p| p.trim().parse().unwrap())
                .collect()
        })
        .enumerate()
        .collect();

    let mut group_count = 0;

    while connections.len() > 0 {
        let mut group = HashSet::new();
        group.insert(connections[0].0);

        let mut changed;
        loop {
            changed = false;
            for &(idx, ref connections) in connections.iter() {
                if group.contains(&idx) {
                    continue;
                }
                for c in connections {
                    if group.contains(&c) {
                        changed = true;
                        group.insert(idx);
                        break;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        connections.retain(|c| !group.contains(&c.0));
        group_count += 1;
    }

    group_count
}

#[test]
fn test_example() {
    let input = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";
    assert_eq!(puzzle(input), 2);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 211);
}
