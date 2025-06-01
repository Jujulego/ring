use crate::traits::Filesystem;
use crate::Error;
use std::path::Path;
use tracing::{instrument, trace};

/// Interacts with local filesystem
#[derive(Debug, Default)]
pub struct LocalFilesystem;

impl LocalFilesystem {
    /// Creates a new instance of local filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
}

impl Filesystem for LocalFilesystem {
    type File = std::fs::File;

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

    #[inline]
    #[instrument(name="filesystem.open", skip_all, fields(adaptator = "filesystem"))]
    fn open(&self, path: &Path) -> Result<Self::File, Error> {
        trace!("open {}", path.display());
        std::fs::File::open(path).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_file_should_detect_directories() {
        assert!(LocalFilesystem.is_dir(Path::new("assets")));

        assert!(!LocalFilesystem.is_dir(Path::new("assets/foo.txt")));
        assert!(!LocalFilesystem.is_dir(Path::new("assets/do-no-exists")));
    }

    #[test]
    fn is_file_should_detect_files() {
        assert!(LocalFilesystem.is_file(Path::new("assets/foo.txt")));

        assert!(!LocalFilesystem.is_file(Path::new("assets")));
        assert!(!LocalFilesystem.is_file(Path::new("assets/do-no-exists")));
    }
}