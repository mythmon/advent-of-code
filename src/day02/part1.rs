use advent::extremes;

fn main() {
    let input: &'static str = include_str!("input");
    println!("{}", puzzle(input));
}

fn puzzle(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let rows: Vec<Vec<u32>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace().map(|s| s.parse().unwrap()).collect()
        })
        .collect();

    let mut sum = 0;

    for row in rows {
        let (min, max): (u32, u32) = extremes(row).unwrap();
        sum += max - min;
    }

    sum
}

#[test]
fn test_examples() {
    let input = "5 1 9 5\n7 5 3\n2 4 6 8\n";
    assert_eq!(puzzle(input), 18);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 34_581);
}
