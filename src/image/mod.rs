use self::header::Header;
use crate::coding::image::ImageCoding;
use pixel_shift::PixelShift;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Bytes, Read, Write};
use std::path::Path;

pub mod compression;
mod header;
mod pixel_shift;

#[derive(PartialEq)]
pub enum ImageType {
    Tga,
    Propra,
}

pub struct Image {
    src_image: ImageType,
    header: Header,
    dest: String,
    buffer: Bytes<BufReader<File>>,
}

impl Image {
    pub fn from_propra(src: ImageCoding) -> Self {
        let file = File::open(src.input_path).unwrap();
        let mut buffer = BufReader::new(file).bytes();

        let mut data: [u8; 30] = [0; 30];
        let mut i = 0;
        while i < data.len() {
            match &buffer.next() {
                Some(value) => match value {
                    Ok(b) => data[i] = *b,
                    Err(_) => (),
                },
                None => (),
            }
            i += 1;
        }

        let mut header = Header::new(src.compression);
        header.from_propra(data);
        Image {
            src_image: ImageType::Propra,
            header,
            dest: src.output_path,
            buffer,
        }
    }

    pub fn to_tga(&mut self) {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(Path::new(&self.dest))
            .unwrap();
        let mut file = BufWriter::new(file);

        let mut data: Vec<u8> = self.header.to_tga().to_vec();

        match self.src_image {
            ImageType::Tga => (),
            ImageType::Propra => {
                for b in PixelShift::from(self) {
                    data.push(b);
                }
            }
        }

        let _ = file.write_all(&data);
        file.flush().unwrap();
    }
}

impl Iterator for Image {
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
