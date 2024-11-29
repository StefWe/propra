use std::{collections::HashMap, iter::Iterator};

pub struct Base32Decode<Iter> {
    buffer: u16,
    index: usize,
    iter: Iter,
    alphabet: HashMap<char, u16>,
}

impl<Iter: Iterator<Item = u8>> Base32Decode<Iter> {
    fn read_next_byte(&mut self) {
        while self.index < 8 {
            match self.iter.next() {
                Some(b) => match self.alphabet.get(&(b as char)) {
                    Some(i) => {
                        self.buffer |= i << ((8 - self.index) + 3);
                        self.index += 5;
                    }
                    None => unreachable!(),
                },
                None => {
                    return;
                }
            }
        }
    }
}

impl<Iter: Iterator> Iterator for Base32Decode<Iter>
where
    Iter: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 8 {
            self.read_next_byte();
        }

        if self.index < 8 {
            return None;
        }

        let result = (self.buffer >> 8) as u8;
        self.buffer <<= 8;
        self.index -= 8;
        Some(result)
    }
}

impl<Iter: Iterator> std::convert::From<Iter> for Base32Decode<Iter> {
    fn from(it: Iter) -> Self {
        Base32Decode {
            buffer: 0,
            index: 0,
            iter: it,
            alphabet: get_alphabet(),
        }
    }
}

fn get_alphabet() -> HashMap<char, u16> {
    HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
        ('G', 16),
        ('H', 17),
        ('I', 18),
        ('J', 19),
        ('K', 20),
        ('L', 21),
        ('M', 22),
        ('N', 23),
        ('O', 24),
        ('P', 25),
        ('Q', 26),
        ('R', 27),
        ('S', 28),
        ('T', 29),
        ('U', 30),
        ('V', 31),
    ])
}
