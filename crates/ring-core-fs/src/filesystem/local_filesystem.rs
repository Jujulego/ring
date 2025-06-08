use crate::traits::{FileMetadata, Filesystem};
use crate::{Error, LocationType};
use std::path::Path;
use tracing::{instrument, trace};

/// Interacts with local filesystem
#[derive(Debug, Default)]
pub struct LocalFilesystem;

impl LocalFilesystem {
    /// Creates a new instance of local filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl FileMetadata for LocalFilesystem {
    #[inline]
    #[instrument(name = "filesystem.location_type", skip_all, fields(adaptator = "filesystem"))]
    fn location_type(&self, path: &Path) -> Result<LocationType, Error> {
        Ok(path.metadata()?.file_type().into())
    }
}

impl Filesystem for LocalFilesystem {
    type File = std::fs::File;

    #[inline]
    #[instrument(name="filesystem.open", skip_all, fields(adaptator = "filesystem"))]
    fn open(&self, path: &Path) -> Result<Self::File, Error> {
        trace!("open {}", path.display());
        std::fs::File::open(path).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::AbsReader;

    #[test]
    fn is_dir_should_detect_directories() {
        assert!(LocalFilesystem.is_dir(Path::new("assets")));

        assert!(!LocalFilesystem.is_dir(Path::new("assets/foo.txt")));
        assert!(!LocalFilesystem.is_dir(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(LocalFilesystem.is_file(Path::new("assets/foo.txt")));

        assert!(!LocalFilesystem.is_file(Path::new("assets")));
        assert!(!LocalFilesystem.is_file(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn open_should_allow_read_file() {
        let mut file = LocalFilesystem.open(Path::new("assets/foo.txt")).unwrap();
        
        assert_eq!(std::io::read_to_string(file.abs_reader()).unwrap(), String::from("bar"));
    }
}