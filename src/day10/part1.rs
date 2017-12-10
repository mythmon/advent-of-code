fn main() {
    let input = get_input();
    println!("{}", puzzle(256, &input));
}

fn get_input() -> Vec<usize> {
    let input: &'static str = include_str!("input");
    input
        .split(",")
        .filter_map(|p| p.trim().parse().ok())
        .collect()
}

fn puzzle(length: usize, instructions: &Vec<usize>) -> usize {
    let k = knot(length, instructions);
    k[0] * k[1]
}

fn knot(length: usize, instructions: &Vec<usize>) -> Vec<usize> {
    let mut items: Vec<usize> = (0..length).collect();
    let mut position = 0;
    let mut skip_size = 0;

    for instr in instructions {
        let mut section: Vec<usize> = if position + instr < length {
            let range = position..(position + instr);
            Vec::from(&items[range])
        } else {
            let mut part1 = Vec::from(&items[position..]);
            let mut part2 = Vec::from(&items[..(position + instr) % length]);
            part1.append(&mut part2);
            part1
        };
        section.reverse();

        for (i, v) in section.into_iter().enumerate() {
            items[(i + position) % length] = v;
        }
        position = (position + instr + skip_size) % length;
        skip_size += 1;
    }

    items
}

#[test]
fn test_knot_example() {
    let instructions = vec![3, 4, 1, 5];
    assert_eq!(knot(5, &instructions), vec![3, 4, 2, 1, 0]);
}

#[test]
fn test_puzzle_example() {
    let instructions = vec![3, 4, 1, 5];
    assert_eq!(puzzle(5, &instructions), 12);
}

#[test]
fn test_correct_answer() {
    let input = get_input();
    assert_eq!(puzzle(256, &input), 37230);
}
