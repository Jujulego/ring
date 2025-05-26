use std::fs::File;
use std::io::Read;

/// Wraps a file
pub trait FileWrapper {
    fn as_reader(&mut self) -> Box<dyn Read + '_>;
}

impl FileWrapper for File {
    fn as_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self)
    }
}
