fn main() {
    let input = get_input();
    println!("{}", puzzle(input));
}

fn get_input() -> &'static str {
    let input: &'static str = include_str!("input");
    input.trim()
}

fn puzzle(input: &str) -> String {
    let length = 256;
    let mut input: Vec<usize> = input.bytes().map(|b| b as usize).collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);
    let mut items: Vec<usize> = (0..length).collect();

    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for c in input.iter() {
            let mut section: Vec<usize> = if position + c < length {
                let range = position..(position + c);
                Vec::from(&items[range])
            } else {
                let mut part1 = Vec::from(&items[position..]);
                let mut part2 = Vec::from(&items[..(position + c) % length]);
                part1.append(&mut part2);
                part1
            };
            section.reverse();

            for (i, v) in section.into_iter().enumerate() {
                items[(i + position) % length] = v;
            }
            position = (position + c + skip_size) % length;
            skip_size += 1;
        }
    }

    let mut dense_hash = Vec::with_capacity(16);
    let mut sparse_hash = items.into_iter();
    for _ in 0..16 {
        let mut chunk = Vec::with_capacity(16);
        for _ in 0..16 {
            chunk.push(sparse_hash.next().unwrap());
        }
        dense_hash.push(chunk.into_iter().fold(0, |acc, x| acc ^ x));
    }

    let mut hex_hash = String::with_capacity(32);
    for item in dense_hash {
        hex_hash += &format!("{:02x}", item);
    }

    hex_hash
}

#[test]
fn test_example_1() {
    let input = "";
    assert_eq!(puzzle(input), "a2582a3a0e66e6e86e3812dcb672a272");
}

#[test]
fn test_example_2() {
    let input = "AoC 2017";
    assert_eq!(puzzle(input), "33efeb34ea91902bb2f59c9920caa6cd");
}

#[test]
fn test_example_3() {
    let input = "1,2,3";
    assert_eq!(puzzle(input), "3efbe78a8d82f29979031a4aa0b16a9d");
}

#[test]
fn test_example_4() {
    let input = "1,2,4";
    assert_eq!(puzzle(input), "63960835bcdc130f0b66d7ff4f6a5a8e");
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(input), "70b856a24d586194331398c7fcfa0aaf");
}
