use std::iter::Iterator;

pub struct PixelPropra<Iter> {
    pixel: [Option<u8>; 3],
    index: usize,
    iter: Iter,
}

impl<Iter: Iterator<Item = u8>> PixelPropra<Iter> {
    fn read_next_pixel(&mut self) {
        self.index = 0;
        while self.index < 3 {
            match self.iter.next() {
                Some(b) => {
                    self.pixel[self.index] = Some(b);
                    self.inc_index();
                }
                None => {
                    self.pixel[0] = None;
                    self.pixel[1] = None;
                    self.pixel[2] = None;
                    return;
                }
            }
        }
    }

    fn inc_index(&mut self) {
        self.index = (self.index + 1) % 4;
    }
}

impl<Iter: Iterator> Iterator for PixelPropra<Iter>
where
    Iter: Iterator<Item = u8>,
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 3 {
            self.read_next_pixel();
            self.pixel.rotate_left(1);
            self.index = 0;
        }
        let i = self.index;
        self.inc_index();
        self.pixel[i]
    }
}

impl<Iter: Iterator> std::convert::From<Iter> for PixelPropra<Iter> {
    fn from(it: Iter) -> Self {
        PixelPropra {
            pixel: [Some(0); 3],
            index: 3,
            iter: it,
        }
    }
}
