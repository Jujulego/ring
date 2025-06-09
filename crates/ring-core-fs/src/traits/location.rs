use crate::FsError;
use std::io::Read;
use std::path::PathBuf;

pub trait Location {
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;
}

impl Location for PathBuf {
    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read>, FsError> {
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}