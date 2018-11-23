#![feature(slice_patterns)]

fn main() {
    println!("{}", puzzle(1));
}

fn puzzle(a: u32) -> u32 {
    let mut composite_count = 0;
    let mut min = 67;
    let mut max = min;

    if a == 1 {
        min *= 100;
        min += 100_000;
        max = min + 17_000;
    }

    for n in (min ..= max + 1).step_by(17) {
        // let max = ((n + 1) as f32).sqrt().ceil() as u32;
        for d in 2..n {
            if n % d == 0 {
                composite_count += 1;
                break;
            }
        }
    }

    composite_count
}

#[test]
fn test_part_1() {
    assert_eq!(puzzle(0), 0);
}

#[test]
fn test_correct_answer() {
    assert_eq!(puzzle(1), 905);
}
