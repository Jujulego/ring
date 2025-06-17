use crate::FsError;
use std::fs::{DirEntry, Metadata};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tracing::trace;

pub trait Location {
    fn path(&self) -> PathBuf;
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;
    fn metadata(&self) -> Option<Result<Metadata, FsError>>;

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
    fn path(&self) -> PathBuf {
        self.clone()
    }

    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read>, FsError> {
        trace!(protocol = "file", "open {}", self.display());
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }

    #[inline]
    fn metadata(&self) -> Option<Result<std::fs::Metadata, FsError>> {
        trace!(protocol = "file", "metadata {}", self.display());
        Some(std::fs::metadata(self).map_err(FsError::from))
    }
}

impl Location for DirEntry {
    #[inline]
    fn path(&self) -> PathBuf {
        self.path()
    }

    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        let path = self.path();

        trace!(protocol = "file", "open {}", path.display());
        std::fs::File::open(path)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }

    #[inline]
    fn metadata(&self) -> Option<Result<Metadata, FsError>> {
        self.path().metadata()
    }
}