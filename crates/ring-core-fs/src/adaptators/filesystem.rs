use std::path::Path;
use crate::PathAdaptator;

/// Access files on the filesystem. Supports any path.
pub struct Filesystem;

impl PathAdaptator for Filesystem {
    #[inline]
    fn is_supported(&self, _: &Path) -> bool {
        true
    }


    #[inline]
    fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        Ok(path.is_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_supported_should_return_true() {
        assert!(Filesystem.is_supported(Path::new("assets/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(Filesystem.is_file(Path::new("assets/foo.txt")).unwrap());

        assert!(!Filesystem.is_file(Path::new("assets")).unwrap());
        assert!(!Filesystem.is_file(Path::new("assets/do-no-exists")).unwrap());
    }
}