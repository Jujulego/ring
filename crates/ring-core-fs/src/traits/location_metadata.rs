use crate::{FsError, LocationType};
use std::path::Path;

pub trait LocationMetadata {
    /// Detects location type
    fn location_type(&self, path: &Path) -> Result<LocationType, FsError>;

    /// Tests if given path exists and is a directory
    #[inline]
    fn is_dir(&self, path: &Path) -> bool {
        self.location_type(path).is_ok_and(|t| t == LocationType::Directory)
    }

    /// Tests if given path exists and is a file
    #[inline]
    fn is_file(&self, path: &Path) -> bool {
        self.location_type(path).is_ok_and(|t| t == LocationType::File)
    }

    /// Tests if given path exists and is a symlink
    #[inline]
    fn is_symlink(&self, path: &Path) -> bool {
        self.location_type(path).is_ok_and(|t| t == LocationType::Symlink)
    }
}