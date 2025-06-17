use std::ffi::OsStr;
use crate::{FsError, LocationType};
use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tracing::trace;

pub trait Location {
    fn path(&self) -> PathBuf;
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;
    fn location_type(&self) -> Result<LocationType, FsError>;

    #[inline]
    fn buf_read(&mut self) -> Result<BufReader<Box<dyn Read + '_>>, FsError> {
        self.read().map(BufReader::new)
    }

    #[inline]
    fn read_to_string(&mut self) -> Result<String, FsError> {
        self.read()
            .and_then(|reader| std::io::read_to_string(reader).map_err(FsError::from))
    }

    #[cfg(feature = "lscolors")]
    #[inline]
    fn indicator(&self) -> lscolors::Indicator {
        lscolors::Indicator::RegularFile
    }

    #[cfg(feature = "lscolors")]
    fn location_style<'a>(&self, ls_colors: &'a lscolors::LsColors) -> Option<&'a lscolors::style::Style> {
        let indicator = self.indicator();
        
        if indicator == lscolors::Indicator::RegularFile {
            let file_style = self.path().file_name()
                .map(OsStr::to_string_lossy)
                .and_then(|name| ls_colors.style_for_str(name.as_ref()));
            
            if let Some(style) = file_style {
                return Some(style)
            }
        }
        
        ls_colors.style_for_indicator(indicator)
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
    fn location_type(&self) -> Result<LocationType, FsError> {
        trace!(protocol = "file", "metadata {}", self.display());
        self.metadata()
            .map(|md| md.file_type().into())
            .map_err(FsError::from)
    }

    #[cfg(feature = "lscolors")]
    fn indicator(&self) -> lscolors::Indicator {
        trace!(protocol = "file", "symlink_metadata {}", self.display());
        let metadata = match std::fs::symlink_metadata(self) {
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

    #[inline]
    fn location_type(&self) -> Result<LocationType, FsError> {
        self.file_type()
            .map(LocationType::from)
            .map_err(FsError::from)
    }

    #[cfg(feature = "lscolors")]
    #[inline]
    fn indicator(&self) -> lscolors::Indicator {
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
            trace!(protocol = "file", "exists {}", self.path().display());
            if matches!(std::fs::exists(self.path()), Ok(true)) {
                lscolors::Indicator::SymbolicLink
            } else {
                lscolors::Indicator::OrphanedSymbolicLink
            }
        } else {
            lscolors::Indicator::RegularFile
        }
    }
}