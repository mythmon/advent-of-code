use std::{error::Error, lazy::Lazy, ops::Deref};

pub struct Recognizer<const W: usize> {
    alphabet: Alphabet<W>,
}

impl<const LETTER_WIDTH: usize> Recognizer<LETTER_WIDTH> {
    #[must_use]
    pub fn new<A>(alphabet: A) -> Self
    where
        A: Deref<Target = Alphabet<LETTER_WIDTH>>,
    {
        Self {
            alphabet: alphabet.clone(),
        }
    }

    /// # Errors
    /// Returns an error if the string passed contains unrecognizable data
    pub fn parse(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let lines: Vec<Vec<char>> = input
            .lines()
            .step_by(self.alphabet.y_stride)
            .map(|line| line.chars().collect())
            .collect();
        let mut result =
            String::with_capacity(lines[0].len() / LETTER_WIDTH / self.alphabet.x_stride);
        let mut char_accumulator = Vec::with_capacity(LETTER_WIDTH / self.alphabet.x_stride);

        // Look at each column in turn
        for cursor in (0..lines[0].len()).step_by(self.alphabet.x_stride) {
            // Pull the column's data from the input as bitstring
            let mut col_data = 0;
            for line in &lines {
                col_data <<= 1;
                if *line.get(cursor).unwrap_or(&' ') != ' ' {
                    col_data |= 1;
                }
            }

            // Skip blank columns between chars
            if col_data == 0 && char_accumulator.is_empty() {
                continue;
            }

            char_accumulator.push(col_data);

            // If we've read an entire character worth of data, interpret it, and possibly
            // error out
            if char_accumulator.len() == LETTER_WIDTH {
                let (letter, _) = self
                    .alphabet
                    .letter_data
                    .iter()
                    .find(|(_, bits)| bits == char_accumulator.as_slice())
                    .ok_or_else(|| format!("Letter not recognized: {:?}", char_accumulator))?;
                result.push(*letter);
                char_accumulator.truncate(0);
            }
        }

        Ok(result)
    }
}

#[derive(Clone, Debug)]
pub struct Alphabet<const LETTER_WIDTH: usize> {
    /// The number of characters each pixel takes up, horizontally
    x_stride: usize,

    /// The number of characters each pixel takes up, vertically
    y_stride: usize,

    // The shapes of characters. Each char is specified as a tuple of that
    // character and a array of the columnar data for that letter. Each pixel in
    // the letter is a bit in the column's number. The least significant bit is
    // the bottom pixel.
    letter_data: Vec<(char, [u16; LETTER_WIDTH])>,
}

#[allow(clippy::declare_interior_mutable_const)]
pub const ALPHABET_2019_D11: Lazy<Alphabet<4>> = Lazy::new(|| Alphabet {
    x_stride: 2,
    y_stride: 1,
    letter_data: vec![
        ('A', [0b01_1111, 0b10_0100, 0b10_0100, 0b01_1111]),
        ('B', [0b11_1111, 0b10_1001, 0b10_1001, 0b01_0110]),
        ('C', [0b01_1110, 0b10_0001, 0b10_0001, 0b01_0010]),
        ('H', [0b11_1111, 0b00_1000, 0b00_1000, 0b11_1111]),
        ('P', [0b11_1111, 0b10_0100, 0b10_0100, 0b01_1000]),
        ('R', [0b11_1111, 0b10_0100, 0b10_0110, 0b01_1001]),
    ],
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2019_d11_one_letter() {
        let grid = vec![
            "██████  ",
            "██    ██",
            "██    ██",
            "██████  ",
            "██  ██  ",
            "██    ██",
        ]
        .join("\n");
        let recognizer = Recognizer::new(ALPHABET_2019_D11);
        assert_eq!(recognizer.parse(&grid).ok(), Some("R".to_string()));
    }

    #[test]
    fn test_2019_d11() {
        let grid = vec![
            "██████      ████    ██████    ██████      ████    ██████    ██████    ██    ██",
            "██    ██  ██    ██  ██    ██  ██    ██  ██    ██  ██    ██  ██    ██  ██    ██",
            "██    ██  ██    ██  ██    ██  ██    ██  ██        ██████    ██    ██  ████████",
            "██████    ████████  ██████    ██████    ██        ██    ██  ██████    ██    ██",
            "██  ██    ██    ██  ██        ██  ██    ██    ██  ██    ██  ██        ██    ██",
            "██    ██  ██    ██  ██        ██    ██    ████    ██████    ██        ██    ██",
        ]
        .join("\n");
        let recognizer = Recognizer::new(ALPHABET_2019_D11);
        assert_eq!(
            recognizer.parse(&grid).expect("Should parse"),
            "RAPRCBPH".to_string()
        );
    }
}
