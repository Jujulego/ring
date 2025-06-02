use std::io::Read;

pub trait AbstractReader {
    fn as_reader(&mut self) -> Box<dyn Read + '_>;
}

impl<R: Read> AbstractReader for R {
    fn as_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self)
    }
}
