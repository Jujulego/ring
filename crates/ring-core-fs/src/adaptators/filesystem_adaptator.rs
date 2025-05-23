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
    #[instrument(name="filesystem.is_file", skip_all, fields(adaptator = "filesystem"))]
    fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        trace!("stat {}", path.display());
        Ok(path.is_file())
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
    fn is_file_should_detect_files() {
        assert!(FilesystemAdaptator.is_file(Path::new("assets/foo.txt")).unwrap());

        assert!(!FilesystemAdaptator.is_file(Path::new("assets")).unwrap());
        assert!(!FilesystemAdaptator.is_file(Path::new("assets/do-no-exists")).unwrap());
    }
}