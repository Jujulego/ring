use crate::error::FsError;
use crate::file_wrapper::FileWrapper;
use std::path::Path;

/// Path based filesystem adaptator
pub trait PathAdaptator {
    /// Tests if given path exists and is a directory
    fn is_dir(&self, path: &Path) -> bool;
    
    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Indicates if the adaptator can handle given path
    fn is_supported(&self, path: &Path) -> bool;

    fn open(&self, path: &Path) -> Result<Box<dyn FileWrapper + '_>, FsError>;
}

