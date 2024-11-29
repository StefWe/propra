pub mod base32decode;
pub mod base32encode;

use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Bytes, Read, Write};
use std::path::Path;

use base32decode::Base32Decode;
use base32encode::Base32Encode;

use crate::coding::base::{BaseCoding, Coding};

pub struct Base {
    dest: String,
    buffer: Bytes<BufReader<File>>,
}

impl Base {
    pub fn produce(src: BaseCoding) {
        match src.coding {
            Coding::DecodeBase32 => Base::base32(src).decode(),
            Coding::EncodeBase32 => Base::base32(src).encode(),
            Coding::DecodeBaseN => panic!("not yet implemented"),
            Coding::EncodeBaseN => panic!("not yet implemented"),
        }
    }

    pub fn base32(src: BaseCoding) -> Self {
        let file = File::open(&src.input_path).unwrap();
        let mut dest = src.input_path;

        match src.coding {
            Coding::DecodeBase32 => dest.truncate(dest.len() - 8),
            Coding::EncodeBase32 => dest.push_str(".base-32"),
            _ => {}
        }

        Base {
            dest,
            buffer: BufReader::new(file).bytes(),
        }
    }

    pub fn decode(&mut self) {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new(&self.dest))
            .unwrap();
        let mut file = BufWriter::new(file);

        let mut data: Vec<u8> = vec![];

        for b in self.base32decode() {
            data.push(b);
        }

        let _ = file.write_all(&data);
        file.flush().unwrap();
    }

    pub fn encode(&mut self) {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new(&self.dest))
            .unwrap();
        let mut file = BufWriter::new(file);

        let mut data: Vec<u8> = vec![];

        for b in self.base32encode() {
            data.push(b);
        }

        let _ = file.write_all(&data);
        file.flush().unwrap();
    }

    fn base32decode(&mut self) -> impl Iterator<Item = u8> + '_ {
        Base32Decode::from(self)
    }

    fn base32encode(&mut self) -> impl Iterator<Item = u8> + '_ {
        Base32Encode::from(self)
    }
}

impl Iterator for Base {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.buffer.next() {
            Some(i) => match i {
                Ok(b) => Some(*b),
                Err(_) => None,
            },
            None => None,
        }
    }
}
