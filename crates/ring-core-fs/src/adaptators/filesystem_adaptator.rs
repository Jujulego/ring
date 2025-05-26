use crate::PathAdaptator;
use std::path::Path;
use tracing::{instrument, trace};

/// Access files on the filesystem. Supports any path.
pub struct FilesystemAdaptator;

impl PathAdaptator for FilesystemAdaptator {
    #[inline]
    fn is_supported(&self, _: &Path) -> bool {
        true
    }

    #[inline]
    #[instrument(name="filesystem.is_dir", skip_all, fields(adaptator = "filesystem"))]
    fn is_dir(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_dir()
    }

    #[inline]
    #[instrument(name="filesystem.is_file", skip_all, fields(adaptator = "filesystem"))]
    fn is_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        path.is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_supported_should_return_true() {
        assert!(FilesystemAdaptator.is_supported(Path::new("assets/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_directories() {
        assert!(FilesystemAdaptator.is_dir(Path::new("assets")));

        assert!(!FilesystemAdaptator.is_dir(Path::new("assets/foo.txt")));
        assert!(!FilesystemAdaptator.is_dir(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(FilesystemAdaptator.is_file(Path::new("assets/foo.txt")));

        assert!(!FilesystemAdaptator.is_file(Path::new("assets")));
        assert!(!FilesystemAdaptator.is_file(Path::new("assets/do-no-exists")));
    }
}