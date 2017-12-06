#![feature(range_contains)]

fn main() {
    let input: &'static str = include_str!("input");
    let input: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("{}", puzzle(input));
}

fn puzzle(mut input: Vec<i32>) -> u32 {
    let mut steps: u32 = 0;
    let mut pc: i32 = 0;

    let bounds = 0..(input.len() as i32);

    while bounds.contains(pc) {
        steps += 1;
        let next = pc + input[pc as usize];
        if input[pc as usize] >= 3 {
            input[pc as usize] -= 1;
        } else {
            input[pc as usize] += 1;
        }
        pc = next;
    }

    steps
}

#[test]
fn test_example() {
    assert_eq!(puzzle(vec![0, 3, 0, 1, -3]), 10);
}

#[test]
fn test_correct_answer() {
    let input: &'static str = include_str!("input");
    let input: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    assert_eq!(puzzle(input), 27763113);
}
