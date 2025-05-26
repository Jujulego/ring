use std::fs::File;
use std::io;

/// Wraps a file
pub trait FileWrapper {
    fn read(&mut self) -> Box<dyn io::Read + '_>;
}

impl FileWrapper for File {
    fn read(&mut self) -> Box<dyn io::Read + '_> {
        Box::new(self)
    }
}
