use crate::Error;
use std::path::Path;

pub trait FilesystemMiddleware {
    type File;

    /// Tests if given path exists and is a directory
    #[inline]
    fn is_dir(&self, _path: &Path) -> Option<bool> {
        None
    }

    /// Tests if given path exists and is a file
    #[inline]
    fn is_file(&self, _path: &Path) -> Option<bool> {
        None
    }

    /// Opens given file
    #[inline]
    fn open(&self, _path: &Path) -> Option<Result<Self::File, Error>> {
        None
    }
}