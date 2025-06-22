use crate::{FsError, LocationType};
use std::ffi::OsString;
use std::fs::DirEntry;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tracing::trace;

pub trait Location {
    /// Returns a [`BufReader`] to access location's contents, if it's a file.
    #[inline]
    fn buf_read(&mut self) -> Result<BufReader<Box<dyn Read + '_>>, FsError> {
        self.read().map(BufReader::new)
    }

    /// Returns a [`lscolors::Indicator`], to compute colors in a "ls" fashioned way.
    #[cfg(feature = "lscolors")]
    #[inline]
    fn indicator(&self) -> lscolors::Indicator {
        lscolors::Indicator::RegularFile
    }

    /// Returns the final component of the location's path
    #[inline]
    fn location_name(&self) -> OsString {
        let path = self.path();

        path.components()
            .next_back()
            .map(|c| c.as_os_str())
            .unwrap_or_else(|| path.as_os_str())
            .to_owned()
    }

    /// Returns location's style based on given [`lscolors::LsColors`] object.
    #[cfg(feature = "lscolors")]
    fn location_style<'a>(&self, ls_colors: &'a lscolors::LsColors) -> Option<&'a lscolors::style::Style> {
        let indicator = self.indicator();

        if indicator == lscolors::Indicator::RegularFile {
            if let Some(style) = ls_colors.style_for_str(self.location_name().to_string_lossy().as_ref()) {
                return Some(style)
            }
        }

        ls_colors.style_for_indicator(indicator)
    }

    /// Returns the location's type (file, folder, symlink ...)
    fn location_type(&self) -> Result<LocationType, FsError>;

    /// Returns the location's path
    fn path(&self) -> PathBuf;

    /// Returns a reader to access location's contents, if it's a file.
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError>;

    /// Reads location's contents to a file and returns it
    #[inline]
    fn read_to_string(&mut self) -> Result<String, FsError> {
        self.read()
            .and_then(|reader| std::io::read_to_string(reader).map_err(FsError::from))
    }
}

impl Location for &Path {
    #[cfg(feature = "lscolors")]
    fn indicator(&self) -> lscolors::Indicator {
        trace!(protocol = "file", "metadata {}", self.display());
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

    #[inline]
    fn location_name(&self) -> OsString {
        self.components()
            .next_back()
            .map(|c| c.as_os_str())
            .unwrap_or_else(|| self.as_os_str())
            .to_owned()
    }

    #[inline]
    fn location_type(&self) -> Result<LocationType, FsError> {
        trace!(protocol = "file", "metadata {}", self.display());
        self.metadata()
            .map(|md| md.file_type().into())
            .map_err(FsError::from)
    }

    #[inline]
    fn path(&self) -> PathBuf {
        self.to_path_buf()
    }

    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read>, FsError> {
        trace!(protocol = "file", "open {}", self.display());
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}

impl Location for PathBuf {
    #[cfg(feature = "lscolors")]
    #[inline]
    fn indicator(&self) -> lscolors::Indicator {
        self.as_path().indicator()
    }

    #[inline]
    fn location_name(&self) -> OsString {
        self.as_path().location_name()
    }

    #[inline]
    fn location_type(&self) -> Result<LocationType, FsError> {
        self.as_path().location_type()
    }

    #[inline]
    fn path(&self) -> PathBuf {
        self.clone()
    }

    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        trace!(protocol = "file", "open {}", self.display());
        std::fs::File::open(self)
            .map(|file| Box::new(file) as _)
            .map_err(FsError::from)
    }
}

impl Location for DirEntry {
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

    #[inline]
    fn location_name(&self) -> OsString {
        self.file_name()
    }

    #[inline]
    fn location_type(&self) -> Result<LocationType, FsError> {
        self.file_type()
            .map(LocationType::from)
            .map_err(FsError::from)
    }

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
}