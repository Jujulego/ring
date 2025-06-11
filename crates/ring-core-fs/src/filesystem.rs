use crate::middlewares::ZipMiddleware;
use crate::protocols::{FileProtocol, MemoryProtocol};
use crate::traits::{FilesystemMiddleware, FilesystemProtocol, Location};
use crate::{FsError, LocationType};
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

/// Combine a protocol with compatible middlewares
pub struct Filesystem {
    protocol: Rc<dyn FilesystemProtocol>,
    middlewares: Vec<Box<dyn FilesystemMiddleware>>,
}

impl Filesystem {
    /// Creates an [`Filesystem`] based on [`FileProtocol`].
    ///
    /// Includes following middlewares:
    /// - [`ZipMiddleware`]
    pub fn local() -> Self {
        let protocol = Rc::new(FileProtocol::new());

        Self {
            protocol: protocol.clone(),
            middlewares: vec![Box::new(ZipMiddleware::new(protocol))]
        }
    }

    /// Creates an [`Filesystem`] based on [`MemoryProtocol`].
    pub fn memory(protocol: MemoryProtocol) -> Self {
        let protocol = Rc::new(protocol);

        Self {
            protocol: protocol.clone(),
            middlewares: vec![]
        }
    }

    /// Tests if given path is a directory. Returns false in case of error
    #[instrument(name="filesystem.is_dir", skip_all)]
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_dir(path.as_ref()))
            .unwrap_or_else(|| self.protocol.is_dir(path.as_ref()))
    }

    /// Tests if given path is a file. Returns false in case of error
    #[instrument(name="filesystem.is_file", skip_all)]
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_file(path.as_ref()))
            .unwrap_or_else(|| self.protocol.is_file(path.as_ref()))
    }

    /// Tests if given path is a symbolic link. Returns false in case of error
    #[instrument(name="filesystem.is_symlink", skip_all)]
    pub fn is_symlink(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_symlink(path))
            .unwrap_or_else(|| self.protocol.is_symlink(path))
    }

    /// Check path existence and return its type
    #[instrument(name="filesystem.location_type", skip_all)]
    pub fn location_type<P: AsRef<Path>>(&self, path: P) -> Result<LocationType, FsError> {
        self.middlewares.iter()
            .find_map(|m| m.maybe_location_type(path.as_ref()))
            .unwrap_or_else(|| self.protocol.location_type(path.as_ref()))
    }

    /// Checks path existence and return a [`Location`] object to interact with it
    #[instrument(name="filesystem.locate_path", skip_all)]
    pub fn locate_path<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn Location + '_>, FsError> {
        self.middlewares.iter()
            .find_map(|m| m.maybe_locate_path(path.as_ref()))
            .unwrap_or_else(|| self.protocol.locate_path(path.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dir_should_detect_directory_in_and_out_archives() {
        let filesystem = Filesystem::local();

        assert!(filesystem.is_dir(Path::new("assets")));
        assert!(!filesystem.is_dir(Path::new("assets/foo.txt")));

        assert!(!filesystem.is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")));
        assert!(filesystem.is_dir(Path::new("assets/yarn-archive.zip/node_modules")));
        assert!(!filesystem.is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
        
        assert!(matches!(filesystem.location_type(Path::new("assets")), Ok(LocationType::Directory)));
    }

    #[test]
    fn is_file_should_detect_file_in_and_out_archives() {
        let filesystem = Filesystem::local();

        assert!(!filesystem.is_file("assets"));
        assert!(filesystem.is_file("assets/foo.txt"));

        assert!(!filesystem.is_file("assets/yarn-archive.zip/do-not-exists"));
        assert!(!filesystem.is_file("assets/yarn-archive.zip/node_modules"));
        assert!(filesystem.is_file("assets/yarn-archive.zip/node_modules/foo.txt"));

        assert!(matches!(filesystem.location_type(Path::new("assets/foo.txt")), Ok(LocationType::File)));
    }

    #[test]
    fn open_should_allow_read_file_in_and_out_archives() {
        let filesystem = Filesystem::local();

        let mut location = filesystem.locate_path("assets/foo.txt").unwrap();

        assert_eq!(location.read_to_string().unwrap(), String::from("bar"));

        let mut location = filesystem.locate_path("assets/yarn-archive.zip/node_modules/foo.txt").unwrap();

        assert_eq!(location.read_to_string().unwrap(), String::from("bar"));
    }
}