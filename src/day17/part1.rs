fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> usize {
    let input: &'static str = include_str!("input");
    input.trim().parse().unwrap()
}

fn puzzle(step: usize) -> u32 {
    let mut buffer = Vec::with_capacity(2018);
    buffer.push(0);
    let mut pos = 0;

    let max = 2018;

    for i in 1..max {
        pos = (pos + step) % buffer.len();
        buffer.insert(pos + 1, i);
        pos += 1;
    }

    buffer[(pos + 1) % buffer.len()]
}

#[test]
fn test_example() {
    let input = 3;
    assert_eq!(puzzle(input), 638);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    puzzle(input);
    assert_eq!(puzzle(input), 1244);
}
