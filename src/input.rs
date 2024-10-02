use crate::coding;
use crate::coding::base::BaseCoding;
use crate::coding::image::ImageCoding;

pub fn check_image_or_base_coding_needed(args: &[String]) -> Result<coding::Type, &'static str> {
    if base_coding_is_needed(args) {
        match BaseCoding::new(args) {
            Ok(s) => return Ok(coding::Type::Base(s)),
            Err(e) => return Err(e),
        };
    }

    match ImageCoding::new(args) {
        Ok(s) => Ok(coding::Type::Image(s)),
        Err(e) => Err(e),
    }
}

fn base_coding_is_needed(args: &[String]) -> bool {
    for s in args {
        if s.contains("code-base-") {
            return true;
        }
    }

    false
}
