use crate::middlewares::ZipMiddleware;
use crate::protocols::{LocalProtocol, MemoryProtocol};
use crate::traits::{FilesystemMiddleware, FilesystemProtocol, Location, LocationMetadata};
use crate::{FsError, LocationType};
use std::path::Path;
use std::rc::Rc;

/// Combine a protocol with compatible middlewares
pub struct Filesystem {
    protocol: Rc<dyn FilesystemProtocol>,
    middlewares: Vec<Box<dyn FilesystemMiddleware>>,
}

impl Filesystem {
    /// Creates an [`Filesystem`] based on [`LocalProtocol`].
    ///
    /// Includes following middlewares:
    /// - [`ZipMiddleware`]
    pub fn local() -> Self {
        let protocol = Rc::new(LocalProtocol::new());

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
}

impl LocationMetadata for Filesystem {
    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn location_type(&self, path: &Path) -> Result<LocationType, FsError> {
        self.middlewares.iter()
            .find_map(|m| m.maybe_location_type(path))
            .unwrap_or_else(|| self.protocol.location_type(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_dir(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_dir(path))
            .unwrap_or_else(|| self.protocol.is_dir(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_file(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_file(path))
            .unwrap_or_else(|| self.protocol.is_file(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_symlink(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_symlink(path))
            .unwrap_or_else(|| self.protocol.is_symlink(path))
    }
}

impl FilesystemProtocol for Filesystem {
    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location + '_>, FsError> {
        self.middlewares.iter()
            .find_map(|m| m.maybe_locate_path(path))
            .unwrap_or_else(|| self.protocol.locate_path(path))
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

        assert!(!filesystem.is_file(Path::new("assets")));
        assert!(filesystem.is_file(Path::new("assets/foo.txt")));

        assert!(!filesystem.is_file(Path::new("assets/yarn-archive.zip/do-not-exists")));
        assert!(!filesystem.is_file(Path::new("assets/yarn-archive.zip/node_modules")));
        assert!(filesystem.is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));

        assert!(matches!(filesystem.location_type(Path::new("assets/foo.txt")), Ok(LocationType::File)));
    }

    #[test]
    fn open_should_allow_read_file_in_and_out_archives() {
        let filesystem = Filesystem::local();

        let mut location = filesystem.locate_path(Path::new("assets/foo.txt")).unwrap();
        let file = location.read().unwrap();

        assert_eq!(std::io::read_to_string(file).unwrap(), String::from("bar"));

        let mut location = filesystem.locate_path(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap();
        let file = location.read().unwrap();

        assert_eq!(std::io::read_to_string(file).unwrap(), String::from("bar"));
    }
}