use crate::filesystem::LocalFilesystem;
use crate::middlewares::ZipMiddleware;
use crate::traits::{AbsFilesystem, AbsMaybeFilesystem, AbsReader, FileMetadata};
use crate::{Error, LocationType};
use std::path::Path;
use std::rc::Rc;

/// Combine a filesystem with some middlewares
pub struct AugmentedFilesystem<F> {
    filesystem: Rc<F>,
    middlewares: Vec<Box<dyn AbsMaybeFilesystem>>,
}

impl<F> AugmentedFilesystem<F> {
    /// Returns filesystem instance
    #[inline]
    pub fn filesystem(&self) -> &Rc<F> {
        &self.filesystem
    }
}

impl AugmentedFilesystem<LocalFilesystem> {
    /// Creates an [`AugmentedFilesystem`] based on [`LocalFilesystem`].
    ///
    /// Includes following middlewares:
    /// - [`ZipMiddleware`]
    pub fn local_filesystem() -> Self {
        let filesystem = Rc::new(LocalFilesystem::new());

        Self {
            filesystem: filesystem.clone(),
            middlewares: vec![Box::new(ZipMiddleware::new(filesystem))]
        }
    }
}

impl<F: FileMetadata> FileMetadata for AugmentedFilesystem<F> {
    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn location_type(&self, path: &Path) -> Result<LocationType, Error> {
        self.middlewares.iter()
            .find_map(|m| m.maybe_location_type(path))
            .unwrap_or_else(|| self.filesystem.location_type(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_dir(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_dir(path))
            .unwrap_or_else(|| self.filesystem.is_dir(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_file(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_file(path))
            .unwrap_or_else(|| self.filesystem.is_file(path))
    }

    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn is_symlink(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.maybe_is_symlink(path))
            .unwrap_or_else(|| self.filesystem.is_symlink(path))
    }
}

impl<F: AbsFilesystem> AbsFilesystem for AugmentedFilesystem<F> {
    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, Error> {
        self.middlewares.iter()
            .find_map(|m| m.abs_maybe_open(path))
            .unwrap_or_else(|| self.filesystem.abs_open(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_dir_should_detect_directory_in_and_out_archives() {
        let filesystem = AugmentedFilesystem::local_filesystem();

        assert!(filesystem.is_dir(Path::new("assets")));
        assert!(!filesystem.is_dir(Path::new("assets/foo.txt")));

        assert!(!filesystem.is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")));
        assert!(filesystem.is_dir(Path::new("assets/yarn-archive.zip/node_modules")));
        assert!(!filesystem.is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_file_in_and_out_archives() {
        let filesystem = AugmentedFilesystem::local_filesystem();

        assert!(!filesystem.is_file(Path::new("assets")));
        assert!(filesystem.is_file(Path::new("assets/foo.txt")));

        assert!(!filesystem.is_file(Path::new("assets/yarn-archive.zip/do-not-exists")));
        assert!(!filesystem.is_file(Path::new("assets/yarn-archive.zip/node_modules")));
        assert!(filesystem.is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
    }

    #[test]
    fn open_should_allow_read_file_in_and_out_archives() {
        let filesystem = AugmentedFilesystem::local_filesystem();

        let mut file = filesystem.abs_open(Path::new("assets/foo.txt")).unwrap();
        assert_eq!(std::io::read_to_string(file.abs_reader()).unwrap(), String::from("bar"));

        let mut file = filesystem.abs_open(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap();
        assert_eq!(std::io::read_to_string(file.abs_reader()).unwrap(), String::from("bar"));
    }
}