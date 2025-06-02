use crate::filesystem::LocalFilesystem;
use crate::middlewares::ZipMiddleware;
use crate::traits::{AbsFilesystem, AbsMaybeFilesystem, AbsReader, FileMetadata};
use crate::Error;
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
}

impl<F: AbsFilesystem> AbsFilesystem for AugmentedFilesystem<F> {
    /// Try each middleware in order, ending with filesystem if every middleware returned [`None`]
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, Error> {
        self.middlewares.iter()
            .find_map(|m| m.abs_maybe_open(path))
            .unwrap_or_else(|| self.filesystem.abs_open(path))
    }
}