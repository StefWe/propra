use std::iter::Iterator;

pub struct PixelShift<Iter> {
    pixel: Vec<u8>,
    iter: Iter,
}

impl<Iter: Iterator<Item = u8>> PixelShift<Iter> {
    fn read_next_pixel(&mut self) -> bool {
        while self.pixel.len() < 3 {
            match self.iter.next() {
                Some(b) => self.pixel.push(b),
                None => return false,
            }
        }
        true
    }
}

impl<Iter: Iterator> Iterator for PixelShift<Iter>
where
    Iter: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel.len() == 0 {
            if self.read_next_pixel() {
                self.pixel.rotate_left(1);
                self.pixel.reverse();
            }
        }
        self.pixel.pop()
    }
}

impl<Iter: Iterator> std::convert::From<Iter> for PixelShift<Iter> {
    fn from(it: Iter) -> Self {
        PixelShift {
            pixel: Vec::with_capacity(3),
            iter: it,
        }
    }
}
