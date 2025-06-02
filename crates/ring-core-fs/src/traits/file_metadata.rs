use std::path::Path;

pub trait FileMetadata {
    /// Tests if given path exists and is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> bool;
}