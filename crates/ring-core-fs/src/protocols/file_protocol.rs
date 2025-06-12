use std::fs::{read_dir, ReadDir};
use crate::middlewares::ZipOrigin;
use crate::traits::{FilesystemProtocol, Location, LocationIterator, LocationMetadata};
use crate::{FsError, LocationType};
use std::path::Path;
use tracing::trace;

/// Interacts with local filesystem
#[derive(Debug, Default)]
pub struct FileProtocol;

impl FileProtocol {
    /// Creates a new instance of local filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl LocationMetadata for FileProtocol {
    #[inline]
    fn location_type(&self, path: &Path) -> Result<LocationType, FsError> {
        trace!(protocol = "file", "metadata {}", path.display());
        Ok(path.metadata()?.file_type().into())
    }
}

impl FilesystemProtocol for FileProtocol {
    #[inline]
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location>, FsError> {
        trace!(protocol = "file", "canonicalize {}", path.display());
        path.canonicalize()
            .map(|p| Box::new(p) as _)
            .map_err(FsError::from)
    }

    #[inline]
    fn read_dir(&self, path: &Path) -> Result<LocationIterator, FsError> {
        trace!(protocol = "file", "read_dir {}", path.display());
        read_dir(path)
            .map(|iter| Box::new(FileIterator::new(iter)) as _)
            .map_err(FsError::from)
    }
}

impl ZipOrigin for FileProtocol {
    type File = std::fs::File;

    #[inline]
    fn open_zip(&self, path: &Path) -> Result<Self::File, FsError> {
        trace!(protocol = "file", "open {}", path.display());
        std::fs::File::open(path).map_err(FsError::from)
    }
}

#[derive(Debug)]
struct FileIterator(Option<ReadDir>);

impl FileIterator {
    #[inline]
    fn new(iter: ReadDir) -> FileIterator {
        FileIterator(Some(iter))
    }
}

impl Iterator for FileIterator {
    type Item = Result<Box<dyn Location>, FsError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut inner = self.0.take()?;
        
        if let Some(entry) = inner.next() {
            self.0 = Some(inner);
            
            Some(entry
                .map(|entry| Box::new(entry) as _)
                .map_err(FsError::from))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn is_dir_should_detect_directories() {
        assert!(FileProtocol.is_dir(Path::new("assets")));

        assert!(!FileProtocol.is_dir(Path::new("assets/foo.txt")));
        assert!(!FileProtocol.is_dir(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(FileProtocol.is_file(Path::new("assets/foo.txt")));

        assert!(!FileProtocol.is_file(Path::new("assets")));
        assert!(!FileProtocol.is_file(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn protocol_should_allow_read_file() {
        let mut location = FileProtocol.locate_path(Path::new("assets/foo.txt")).unwrap();
        
        assert_eq!(location.read_to_string().unwrap(), String::from("bar"));
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn protocol_should_allow_read_directory_content() {
        let locations = FileProtocol.read_dir(Path::new("assets")).unwrap()
            .map(|location| location.unwrap().path())
            .collect::<Vec<_>>();

        assert_eq!(locations, vec![PathBuf::from(r"assets\foo.txt"), PathBuf::from(r"assets\yarn-archive.zip")]);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn protocol_should_allow_read_directory_content() {
        let locations = FileProtocol.read_dir(Path::new("assets")).unwrap()
            .map(|location| location.unwrap().path())
            .collect::<Vec<_>>();

        assert_eq!(locations, vec![PathBuf::from("assets/foo.txt"), PathBuf::from("assets/yarn-archive.zip")]);
    }
}