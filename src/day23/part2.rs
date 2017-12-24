#![feature(match_default_bindings)]
#![feature(slice_patterns)]

extern crate advent;

fn main() {
    println!("{}", puzzle(1));
}

fn puzzle(a: u32) -> u32 {
    let mut composite_count = 0;
    let min;
    let max;

    if a == 0 {
        min = 67;
        max = 67;
    } else {
        min =  106_700;
        max =  113_700;
    }
    for n in min..max {
        for divisor in 2..n {
            if n % divisor == 0 {
                println!("{} is divisable by {} ({0} / {1} = {}, {0} % {1} = {}, {})", n, divisor, n / divisor, n % divisor, composite_count);
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
    let v = puzzle(1);
    assert_ne!(v, 1);
    assert!(v < 6407);
}
