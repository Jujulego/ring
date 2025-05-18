use std::path::Path;
use crate::PathAdaptator;

/// Access files on the filesystem
pub struct Filesystem;

impl PathAdaptator for Filesystem {
    #[inline]
    fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        Ok(path.is_file())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn is_file_should_return_true() {
        assert!(Filesystem.is_file(Path::new("assets/foo.txt")).unwrap());
    }
}