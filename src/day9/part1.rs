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

    let mut total_score = 0;
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
            (Some(InGroup(v)), '}') => {
                state_stack.pop();
                total_score += v;
            }
            (Some(InGroup(_)), ',') => (),
            (Some(InGroup(_)), '<') => {
                state_stack.push(Garbage);
            }
            (Some(Garbage), '>') => {
                state_stack.pop();
            }
            (Some(Garbage), _) => (),

            _ => panic!("unexpected input '{}' in {:?}", c, state),
        }
    }

    assert_eq!(state_stack.len(), 0);

    total_score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseState {
    InGroup(u32),
    Garbage,
    Cancel,
}

#[test]
fn test_example_1() {
    let input = "{}";
    assert_eq!(puzzle(input), 1);
}

#[test]
fn test_example_2() {
    let input = "{{{}}}";
    assert_eq!(puzzle(input), 6);
}

#[test]
fn test_example_3() {
    let input = "{{},{}}";
    assert_eq!(puzzle(input), 5);
}

#[test]
fn test_example_4() {
    let input = "{{{},{},{{}}}}";
    assert_eq!(puzzle(input), 16);
}

#[test]
fn test_example_5() {
    let input = "{<a>,<a>,<a>,<a>}";
    assert_eq!(puzzle(input), 1);
}

#[test]
fn test_example_6() {
    let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
    assert_eq!(puzzle(input), 9);
}

#[test]
fn test_example_7() {
    let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    assert_eq!(puzzle(input), 9);
}

#[test]
fn test_example_8() {
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(puzzle(input), 3);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), 17390);
}
