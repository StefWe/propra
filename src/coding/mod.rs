pub mod base;
pub mod image;

use crate::coding::base::BaseCoding;
use crate::coding::image::ImageCoding;

pub enum Type {
    Image(ImageCoding),
    Base(BaseCoding),
}
