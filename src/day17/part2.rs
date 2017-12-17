fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> usize {
    let input: &'static str = include_str!("input");
    input.trim().parse().unwrap()
}

fn puzzle(step: usize) -> u32 {
    let max = 50_000_000;

    let mut val_after_0 = None;
    let mut pos = 0;
    let mut current_length = 1;

    for i in 1..max {
        pos = (pos + step) % current_length;
        if pos == 0 {
            val_after_0 = Some(i);
        }
        current_length += 1;
        pos += 1;
    }

    val_after_0.unwrap()
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    assert_eq!(puzzle(input), 11162912);
}
