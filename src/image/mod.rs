use self::header::Header;
use crate::coding::image::ImageCoding;
use std::fs::File;
use std::io::BufReader;
use std::io::Bytes;
use std::io::Read;

pub mod compression;
mod header;

#[derive(PartialEq)]
pub enum ImageType {
    Tga,
    Propra,
}

pub struct Image {
    src_image: ImageType,
    header: Header,
    buffer: BufReader<File>,
}

impl Image {
    pub fn from_propra(src: ImageCoding) -> Self {
        //pub fn from_propra(src: ImageCoding) -> impl Iterator<Item = Result<u8, Error>> {
        //let mut buffer: [u8; 9000] = [0; 9000];
        let file = File::open(src.input_path).unwrap();
        //let mut reader = BufReader::new(f);

        //Image::new(BufReader::new(file).bytes())

        /*for b in BufReader::new(&file).bytes() {
            print!("");
        }*/

        Image {
            src_image: ImageType::Propra,
            header: Header::new(src.compression),
            buffer: BufReader::new(file),
        }

        //BufReader::new(file).bytes()
        /*
        match result {
            std::result::Result::Ok(f) => BufReader::new(f).bytes(),
            std::result::Result::Err(e) => std::io::Error::new(e),
        }*/

        /*
        let n = f.read(&mut buffer[..]);
        let mut bytes_in_buffer = match n {
            std::result::Result::Ok(n) => n,
            std::result::Result::Err(_) => 0,
        };
        //let mut n = f.read(&mut buffer[..]);
        //
        BufReader::new(f).bytes()*/
    }

    pub fn to_tga(&mut self) {
        for b in self.buffer.bytes() {
            println!("{:#?}", b);
        }
        //BufWrite(self.pixel_shift(self.header.to_tga(self.iter)));
    }
}
