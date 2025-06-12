use std::fs::DirEntry;
use crate::FsError;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tracing::trace;

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
        trace!(protocol = "file", "open {}", self.display());
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}

impl Location for DirEntry {
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        let path = self.path();

        trace!(protocol = "file", "open {}", path.display());
        std::fs::File::open(path)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}