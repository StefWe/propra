use crate::image::compression::Compression;

#[derive(Debug)]
pub struct ImageCoding {
    pub input_path: String,
    pub output_path: String,
    pub compression: Compression,
}

impl ImageCoding {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        let input_path = match ImageCoding::set_input_path(args) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let output_path = match ImageCoding::set_output_path(args) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let compression = match ImageCoding::set_compression(args) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };

        if compression == Compression::Huffman && output_path.ends_with(".tga") {
            return Err("Huffman Codierung wird für tga Bildformat nicht unterstützt.");
        }

        Ok(ImageCoding {
            input_path,
            output_path,
            compression,
        })
    }

    fn set_input_path(args: &Vec<String>) -> Result<String, &'static str> {
        for s in args.iter() {
            match s {
                s if s.starts_with("--input=") => return Ok(s.clone().split_off(8)),
                _ => (),
            }
        }
        Err("Kein --input Parameter gefunden")
    }

    fn set_output_path(args: &Vec<String>) -> Result<String, &'static str> {
        for s in args.iter() {
            match s {
                s if s.starts_with("--output=") => return Ok(s.clone().split_off(9)),
                _ => (),
            }
        }
        Err("Kein --output Parameter gefunden")
    }

    fn set_compression(args: &Vec<String>) -> Result<Compression, &'static str> {
        for s in args.iter() {
            match s {
                s if s.eq("--compression=huffman") => return Ok(Compression::Huffman),
                s if s.eq("--compression=rle") => return Ok(Compression::Rle),
                s if s.eq("--compression=uncompressed") => return Ok(Compression::Uncompressed),
                s if s.eq("--compression=auto") => return Ok(Compression::Auto),
                s if s.starts_with("--compression=") => {
                    return Err("Keine gültige Kompression angegeben")
                }
                _ => (),
            }
        }
        Ok(Compression::Uncompressed)
    }
}
