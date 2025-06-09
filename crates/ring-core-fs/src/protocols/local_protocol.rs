use crate::middlewares::ZipOrigin;
use crate::traits::{FilesystemProtocol, Location, LocationMetadata};
use crate::{FsError, LocationType};
use std::path::Path;
use tracing::trace;

/// Interacts with local filesystem
#[derive(Debug, Default)]
pub struct LocalProtocol;

impl LocalProtocol {
    /// Creates a new instance of local filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LocationMetadata for LocalProtocol {
    #[inline]
    fn location_type(&self, path: &Path) -> Result<LocationType, FsError> {
        Ok(path.metadata()?.file_type().into())
    }
}

impl FilesystemProtocol for LocalProtocol {
    #[inline]
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location>, FsError> {
        trace!("canonicalize {}", path.display());
        path.canonicalize()
            .map(|p| Box::new(p) as _)
            .map_err(FsError::from)
    }
}

impl ZipOrigin for LocalProtocol {
    type File = std::fs::File;

    #[inline]
    fn open_zip(&self, path: &Path) -> Result<Self::File, FsError> {
        std::fs::File::open(path).map_err(FsError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dir_should_detect_directories() {
        assert!(LocalProtocol.is_dir(Path::new("assets")));

        assert!(!LocalProtocol.is_dir(Path::new("assets/foo.txt")));
        assert!(!LocalProtocol.is_dir(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(LocalProtocol.is_file(Path::new("assets/foo.txt")));

        assert!(!LocalProtocol.is_file(Path::new("assets")));
        assert!(!LocalProtocol.is_file(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn open_should_allow_read_file() {
        let mut location = LocalProtocol.locate_path(Path::new("assets/foo.txt")).unwrap();
        let file = location.read().unwrap();
        
        assert_eq!(std::io::read_to_string(file).unwrap(), String::from("bar"));
    }
}