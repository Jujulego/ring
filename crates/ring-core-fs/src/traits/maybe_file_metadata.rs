use std::path::Path;

pub trait MaybeFileMetadata {
    /// Tests if given path exists and is a directory
    #[inline]
    #[allow(unused_variables)]
    fn maybe_is_dir(&self, path: &Path) -> Option<bool> {
        None
    }

    /// Tests if given path exists and is a file
    #[inline]
    #[allow(unused_variables)]
    fn maybe_is_file(&self, path: &Path) -> Option<bool> {
        None
    }
}