use crate::Error;
use std::fs::File;
use std::io::Read;

/// Wraps a file
pub trait FileWrapper {
    fn reader(&mut self) -> Result<Box<dyn Read + '_>, Error>;
}

impl FileWrapper for File {
    fn reader(&mut self) -> Result<Box<dyn Read + '_>, Error> {
        Ok(Box::new(self))
    }
}
