use super::ImageType;

#[derive(Debug, PartialEq, Eq)]
pub enum Compression {
    Uncompressed,
    Rle,
    Huffman,
    Auto,
}

impl Compression {
    pub fn get_value(&self, image_type: &ImageType) -> u8 {
        match (self, image_type) {
            (Compression::Uncompressed, ImageType::Tga) => 2,
            (Compression::Uncompressed, ImageType::Propra) => 0,
            (Compression::Rle, ImageType::Tga) => 10,
            (Compression::Rle, ImageType::Propra) => 1,
            (Compression::Huffman, ImageType::Tga) => panic!("Compression not supported"),
            (Compression::Huffman, ImageType::Propra) => 2,
            (Compression::Auto, ImageType::Tga) => panic!("Compression auto not implemented"),
            (Compression::Auto, ImageType::Propra) => panic!("Compression auto not implemented"),
        }
    }
}
