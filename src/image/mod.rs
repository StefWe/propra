use self::header::Header;
use crate::coding::image::ImageCoding;
use crate::CheckSum;
use propra::PixelPropra;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Bytes, Read, Write};
use std::path::Path;
use tga::PixelTga;

pub mod compression;
mod header;
mod propra;
mod tga;

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
    pub fn produce(src: ImageCoding) {
        let from_tga = src.input_path.ends_with(".tga");
        let to_tga = src.output_path.ends_with(".tga");

        match (from_tga, to_tga) {
            (false, false) => Image::from_propra(src).to_propra(),
            (false, true) => Image::from_propra(src).to_tga(),
            (true, false) => Image::from_tga(src).to_propra(),
            (true, true) => Image::from_tga(src).to_tga(),
        }
    }

    pub fn from_tga(src: ImageCoding) -> Self {
        let file = File::open(src.input_path).unwrap();
        let mut buffer = BufReader::new(file).bytes();

        let mut data: [u8; 18] = [0; 18];
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
        header.from_tga(data);
        Image {
            src_image: ImageType::Tga,
            header,
            dest: src.output_path,
            buffer,
        }
    }

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
            .truncate(true)
            .open(Path::new(&self.dest))
            .unwrap();
        let mut file = BufWriter::new(file);

        let mut data: Vec<u8> = self.header.to_tga().to_vec();
        match self.src_image {
            ImageType::Tga => {
                for b in self {
                    data.push(b);
                }
            }
            ImageType::Propra => {
                for b in self.pixel_from_propra_to_tga() {
                    data.push(b);
                }
            }
        }
        let _ = file.write_all(&data);
        file.flush().unwrap();
    }

    pub fn to_propra(&mut self) {
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new(&self.dest))
            .unwrap();
        let mut file = BufWriter::new(file);

        let mut data: Vec<u8> = self.header.to_propra().to_vec();
        let mut checksum = CheckSum::new();

        match self.src_image {
            ImageType::Tga => {
                for b in self.pixel_from_tga_to_propra() {
                    checksum.add(b);
                    data.push(b);
                }
            }
            ImageType::Propra => {
                for b in self {
                    checksum.add(b);
                    data.push(b);
                }
            }
        }

        let sum = checksum.calc();
        data[26] = sum as u8;
        data[27] = (sum >> 8) as u8;
        data[28] = (sum >> 16) as u8;
        data[29] = (sum >> 24) as u8;

        let _ = file.write_all(&data);
        file.flush().unwrap();
    }

    fn pixel_from_propra_to_tga(&mut self) -> impl Iterator<Item = u8> + '_ {
        PixelPropra::from(self)
    }

    fn pixel_from_tga_to_propra(&mut self) -> impl Iterator<Item = u8> + '_ {
        PixelTga::from(self)
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
