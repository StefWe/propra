/*Image Header

* TGA 18 Bytes großer Header
    data[0] = 0 id
    data[1] = 0 colorPaletteType
    data[2] = imageType -> (2=uncompressed / 10=rle)
    data[3..4] = 0 palletStart
    data[5..6] = 0 palletLength
    data[7] = 0    paletteEntrySizeInBits
    data[8..9] = 0 zeroX
    data[10..11] = (height) zeroY //LittleEndian
    data[12..13] = width  //LittleEndian
    data[14..15] = height //LittleEndian
    data[16] = bitsPerPixel //default -> 24bit
    data[17] = attribute if != 32 "Nicht unterstützter Bild Nullpunkt. Vertikale Lage des Nullpunkts muss oben sein."

* Propra 30 Bytes großer Header
*   Format "ProPraWiSe22"
    data[0..11] = "ProPraWiSe22"
    data[12] = (0=uncompressed / 1=rle / 2=huffman)
    data[13..14] = width
    data[15..16] = height
    data[17] = bitsPerPixel //default -> 24bit
    data[18..25] = data_segment_length
    data[26..29] = checksum
*/

use super::compression::Compression;
use super::ImageType;

pub struct Header {
    compression: Compression,
    pub width: u16,
    pub height: u16,
}

impl Header {
    pub fn from_propra(compression: Compression, data: [u8; 30]) -> Self {
        let mut header = Header::new(compression);
        header.compression = if data[12] == 1 {
            Compression::Rle
        } else if data[12] == 2 {
            Compression::Huffman
        } else {
            Compression::Uncompressed
        };
        header.width = ((data[14] as u16) << 8) | data[13] as u16;
        header.height = ((data[16] as u16) << 8) | data[15] as u16;
        header
    }

    pub fn from_tga(compression: Compression, data: [u8; 18]) -> Self {
        let mut header = Header::new(compression);
        header.compression = if data[2] == 10 {
            Compression::Rle
        } else {
            Compression::Uncompressed
        };
        header.width = ((data[13] as u16) << 8) | data[12] as u16;
        header.height = ((data[15] as u16) << 8) | data[14] as u16;
        header
    }

    pub fn to_tga(&self) -> [u8; 18] {
        let bits_per_pixel = 3 * 8;
        let mut data: [u8; 18] = [0; 18];
        data[2] = self.compression.get_value(&ImageType::Tga);
        data[10] = self.height as u8;
        data[11] = (self.height >> 8) as u8;
        data[12] = self.width as u8;
        data[13] = (self.width >> 8) as u8;
        data[14] = self.height as u8;
        data[15] = (self.height >> 8) as u8;
        data[16] = bits_per_pixel;
        data[17] = 32;
        data
    }

    pub fn to_propra(&self) -> [u8; 30] {
        let bits_per_pixel = 3 * 8;
        let mut data: [u8; 30] = [0; 30];
        data[..12].copy_from_slice("ProPraWiSe22".as_bytes());
        data[12] = self.compression.get_value(&ImageType::Propra);
        data[13] = self.width as u8;
        data[14] = (self.width >> 8) as u8;
        data[15] = self.height as u8;
        data[16] = (self.height >> 8) as u8;
        data[17] = bits_per_pixel;
        let data_segment_length: u64 =
            self.width as u64 * self.height as u64 * (bits_per_pixel / 8) as u64;
        data[18] = data_segment_length as u8;
        data[19] = (data_segment_length >> 8) as u8;
        data[20] = (data_segment_length >> 8) as u8;
        data[21] = (data_segment_length >> 8) as u8;
        data[22] = (data_segment_length >> 8) as u8;
        data[23] = (data_segment_length >> 8) as u8;
        data[24] = (data_segment_length >> 8) as u8;
        data[25] = (data_segment_length >> 8) as u8;
        //data[26..29] = checksum
        data
    }

    fn new(compression: Compression) -> Self {
        Header {
            compression,
            width: 0,
            height: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_to_tga_one_pixel_uncompressed_image() {
        let mut header = Header::new(Compression::Uncompressed);
        header.width = 1;
        header.height = 1;
        let result: [u8; 18] = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 24, 32];
        assert_eq!(result, header.to_tga());
    }

    #[test]
    fn check_to_tga_uncompressed_image() {
        let mut header = Header::new(Compression::Uncompressed);
        header.width = 500;
        header.height = 593;
        let result: [u8; 18] = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 81, 2, 244, 1, 81, 2, 24, 32];
        assert_eq!(result, header.to_tga());
    }

    #[test]
    fn check_from_tga_one_pixel_uncompressed_image() {
        let data: [u8; 18] = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 24, 32];
        let header = Header::from_tga(Compression::Rle, data);
        assert_eq!(1, header.width * header.height);
        assert!(header.compression == Compression::Uncompressed);
    }

    #[test]
    fn check_from_tga_uncompressed_image() {
        let data: [u8; 18] = [0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 81, 2, 244, 1, 81, 2, 24, 32];
        let header = Header::from_tga(Compression::Rle, data);
        assert_eq!(500, header.width);
        assert_eq!(593, header.height);
        assert!(header.compression == Compression::Uncompressed);
    }

    #[test]
    fn check_from_tga_rle_image() {
        let data: [u8; 18] = [0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 81, 2, 244, 1, 81, 2, 24, 32];
        let header = Header::from_tga(Compression::Uncompressed, data);
        assert_eq!(500, header.width);
        assert_eq!(593, header.height);
        assert!(header.compression == Compression::Rle);
    }
}
