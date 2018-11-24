pub mod part1;
pub mod part2;

pub struct KnotHash {
    hash_parts: Vec<usize>,
}

impl KnotHash {
    pub fn new(input: &str) -> Self {
        let length = 256;
        let mut input: Vec<usize> = input.bytes().map(|b| b as usize).collect();
        input.append(&mut vec![17, 31, 73, 47, 23]);
        let mut hash_parts: Vec<usize> = (0..length).collect();

        let mut position = 0;
        let mut skip_size = 0;

        for _ in 0..64 {
            for c in input.iter() {
                let mut section: Vec<usize> = if position + c < length {
                    let range = position..(position + c);
                    Vec::from(&hash_parts[range])
                } else {
                    let mut part1 = Vec::from(&hash_parts[position..]);
                    let mut part2 = Vec::from(&hash_parts[..(position + c) % length]);
                    part1.append(&mut part2);
                    part1
                };
                section.reverse();

                for (i, v) in section.into_iter().enumerate() {
                    hash_parts[(i + position) % length] = v;
                }
                position = (position + c + skip_size) % length;
                skip_size += 1;
            }
        }

        Self {
            hash_parts: hash_parts,
        }
    }

    pub fn dense(&self) -> Vec<usize> {
        let mut sparse_hash = self.hash_parts.iter();

        let mut dense_hash = Vec::with_capacity(16);
        for _ in 0..16 {
            let mut chunk = Vec::with_capacity(16);
            for _ in 0..16 {
                chunk.push(sparse_hash.next().unwrap());
            }
            dense_hash.push(chunk.into_iter().fold(0, |acc, x| acc ^ x));
        }

        dense_hash
    }

    pub fn hex(&self) -> String {
        let mut hex_hash = String::with_capacity(32);
        for item in self.dense() {
            hex_hash += &format!("{:02x}", item);
        }
        hex_hash
    }
}
