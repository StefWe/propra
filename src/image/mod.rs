use self::header::Header;

pub mod compression;
mod header;

#[derive(PartialEq)]
pub enum ImageType {
    Tga,
    Propra,
}

pub struct Image {
    header: Header,
    data: i32,
}
