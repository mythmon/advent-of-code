extern crate advent;

use advent::day9::ParseState;

fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input
}

fn puzzle(input: &str) -> u32 {
    use ParseState::*;

    let mut garbage_count = 0;
    let mut state_stack = vec![];

    for c in input.trim().chars() {
        let state = state_stack.last().map(|s| *s);
        match (state, c) {
            (Some(s), '!') if s != Cancel => {
                state_stack.push(Cancel);
            }
            (Some(Cancel), _) => {
                state_stack.pop();
            }
            (None, '{') => {
                state_stack.push(InGroup(1));
            }
            (Some(InGroup(v)), '{') => {
                state_stack.push(InGroup(v + 1));
            }
            (Some(InGroup(_)), '}') => {
                state_stack.pop();
            }
            (Some(InGroup(_)), ',') => (),
            (Some(Garbage), '>') => {
                state_stack.pop();
            }
            (Some(Garbage), _) => {
                garbage_count += 1;
            }
            (_, '<') => {
                state_stack.push(Garbage);
            }

            _ => panic!("unexpected input '{}' in {:?}", c, state),
        }
    }

    assert_eq!(state_stack.len(), 0);

    garbage_count
}


#[test]
fn test_example_1() {
    let input = "<>";
    assert_eq!(puzzle(input), 0);
}

#[test]
fn test_example_2() {
    let input = "<random characters>";
    assert_eq!(puzzle(input), 17);
}

#[test]
fn test_example_3() {
    let input = "<<<<>";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_example_4() {
    let input = "<{!>}>";
    assert_eq!(puzzle(input), 2);
}

#[test]
fn test_example_5() {
    let input = "<!!>";
    assert_eq!(puzzle(input), 0);
}

#[test]
fn test_example_6() {
    let input = "<!!!>>";
    assert_eq!(puzzle(input), 0)
}

#[test]
fn test_example_7() {
    let input = "<{o\"i!a,<{i<a>";
    assert_eq!(puzzle(input), 10);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 7825);
}
