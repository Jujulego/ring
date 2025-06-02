use std::io::Read;

pub trait AbsReader {
    fn abs_reader(&mut self) -> Box<dyn Read + '_>;
}

impl<R: Read> AbsReader for R {
    #[inline]
    fn abs_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self)
    }
}
