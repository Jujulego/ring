use crate::Error;
use std::path::Path;

pub trait Filesystem {
    type File;

    /// Tests if given path exists and is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Opens given file
    fn open(&self, path: &Path) -> Result<Self::File, Error>;
}