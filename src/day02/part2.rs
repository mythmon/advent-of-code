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

    'row: for row in rows {
        for (i, first) in row.iter().enumerate() {
            for second in row[(i + 1)..].iter() {
                let (small, big) = if first < second {
                    (first, second)
                } else {
                    (second, first)
                };
                if big % small == 0 {
                    sum += big / small;
                    continue 'row;
                }
            }
        }
        panic!(format!("Could not find divisible pair in {:?}", row));
    }

    sum
}

#[test]
fn test_examples() {
    let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    assert_eq!(puzzle(input), 9);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input");
    assert_eq!(puzzle(input), 214);
}
