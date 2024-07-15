use crate::coding;
use crate::coding::base::BaseCoding;
use crate::coding::image::ImageCoding;

pub fn validation(args: &Vec<String>) -> Result<coding::Type, &'static str> {
    let mut result: Result<coding::Type, &'static str>;
    match ImageCoding::new(&args) {
        Ok(s) => return Ok(coding::Type::Image(s)),
        Err(e) => result = Err(e),
    };

    match BaseCoding::new(&args) {
        Ok(s) => return Ok(coding::Type::Base(s)),
        Err(e) => result = Err(e),
    };

    result
}
