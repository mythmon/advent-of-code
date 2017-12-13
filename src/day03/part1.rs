fn main() {
    let input = 289326;
    println!("{}", puzzle(input));
}

fn puzzle(input: u32) -> u32 {
    let mut layer = 0;
    let mut layer_max = 0;

    for i in 0.. {
        let odd = i * 2 + 1;
        layer_max = odd * odd;
        if input <= layer_max {
            layer = i;
            break;
        }
    }

    let mut center = layer_max - layer;
    for _ in 0..4 {
        if input >= center - layer {
            let offset = ((center as i32) - (input as i32)).abs() as u32;
            return layer + offset;
        }
        center -= layer * 2;
    }

    unreachable!("should have returned by now");
}

#[test]
fn test_example_1() {
    assert_eq!(puzzle(1), 0);
}

#[test]
fn test_example_2() {
    assert_eq!(puzzle(12), 3);
}

#[test]
fn test_example_3() {
    assert_eq!(puzzle(23), 2);
}

#[test]
fn test_example_4() {
    assert_eq!(puzzle(1024), 31);
}

#[test]
fn correct_answer() {
    assert_eq!(puzzle(289326), 419);
}
