pub mod base;
pub mod image;

use self::base::BaseCoding;
use self::image::ImageCoding;

pub enum Type {
    Image(ImageCoding),
    Base(BaseCoding),
}
