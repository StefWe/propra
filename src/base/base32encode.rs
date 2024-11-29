use std::{collections::HashMap, iter::Iterator};

pub struct Base32Encode<Iter> {
    buffer: u16,
    index: usize,
    iter: Iter,
    alphabet: HashMap<u8, u8>,
}

impl<Iter: Iterator<Item = u8>> Base32Encode<Iter> {
    fn read_next_byte(&mut self) {
        while self.index < 5 {
            match self.iter.next() {
                Some(b) => {
                    let elem = b as u16;
                    self.buffer |= elem << (8 - self.index);
                    self.index += 8;
                }
                None => return,
            }
        }
    }
}

impl<Iter: Iterator> Iterator for Base32Encode<Iter>
where
    Iter: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 5 {
            self.read_next_byte();
        }

        if self.index == 0 {
            return None;
        }

        let result = self.alphabet.get(&((self.buffer >> 11) as u8)).unwrap();
        self.buffer <<= 5;
        self.index = self.index.saturating_sub(5);
        Some(*result)
    }
}

impl<Iter: Iterator> std::convert::From<Iter> for Base32Encode<Iter> {
    fn from(it: Iter) -> Self {
        Base32Encode {
            buffer: 0,
            index: 0,
            iter: it,
            alphabet: get_alphabet(),
        }
    }
}

fn get_alphabet() -> HashMap<u8, u8> {
    HashMap::from([
        (0, 48),
        (1, 49),
        (2, 50),
        (3, 51),
        (4, 52),
        (5, 53),
        (6, 54),
        (7, 55),
        (8, 56),
        (9, 57),
        (10, 65),
        (11, 66),
        (12, 67),
        (13, 68),
        (14, 69),
        (15, 70),
        (16, 71),
        (17, 72),
        (18, 73),
        (19, 74),
        (20, 75),
        (21, 76),
        (22, 77),
        (23, 78),
        (24, 79),
        (25, 80),
        (26, 81),
        (27, 82),
        (28, 83),
        (29, 84),
        (30, 85),
        (31, 86),
    ])
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_first_five_bits_result_should_be_160() {
        let mut source = 0b01010101u8;
        assert_eq!(10, source >> 3);
        assert_eq!(85, source);
        source <<= 5;
        assert_eq!(160, source);
    }

    #[test]
    fn read_first_five_bits_result_should_be_128() {
        let mut source = 0b11001100u8;
        assert_eq!(25, source >> 3);
        assert_eq!(204, source);
        source <<= 5;
        assert_eq!(128, source);
    }
}
