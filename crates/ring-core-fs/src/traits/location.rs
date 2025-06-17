use crate::FsError;
use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tracing::trace;

pub trait Location {
    fn path(&self) -> PathBuf;
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;

    #[cfg(feature = "lscolors")]
    fn indicator(&self) -> lscolors::Indicator {
        lscolors::Indicator::RegularFile
    }

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

    #[cfg(feature = "lscolors")]
    fn indicator(&self) -> lscolors::Indicator {
        trace!(protocol = "file", "metadata {}", self.display());
        let metadata = match self.metadata() {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return lscolors::Indicator::MissingFile
            },
            Err(_) => return lscolors::Indicator::RegularFile,
        };

        if metadata.is_file() {
            lscolors::Indicator::RegularFile
        } else if metadata.is_dir() {
            lscolors::Indicator::Directory
        } else if metadata.is_symlink() {
            trace!(protocol = "file", "exists {}", self.display());
            if matches!(std::fs::exists(self), Ok(true)) {
                lscolors::Indicator::SymbolicLink
            } else {
                lscolors::Indicator::OrphanedSymbolicLink
            }
        } else {
            lscolors::Indicator::RegularFile
        }
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

    #[cfg(feature = "lscolors")]
    #[inline]
    fn indicator(&self) -> lscolors::Indicator {
        self.path().indicator()
    }
}