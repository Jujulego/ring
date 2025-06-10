use crate::FsError;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub trait Location {
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;

    #[inline]
    fn buf_read(&mut self) -> Result<BufReader<Box<dyn Read + '_>>, FsError> {
        self.read().map(BufReader::new)
    }

    #[inline]
    fn read_to_string(&mut self) -> Result<String, FsError> {
        self.read()
            .and_then(|reader| std::io::read_to_string(reader).map_err(FsError::from))
    }
}

impl Location for PathBuf {
    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read>, FsError> {
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}