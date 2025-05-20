use crate::PathAdaptator;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use tracing::{instrument, trace};
use zip::ZipArchive;

/// Access files in yarn archives. Supports yarn virtual paths.
pub struct YarnArchives;

impl PathAdaptator for YarnArchives {
    #[inline]
    fn is_supported(&self, path: &Path) -> bool {
        path.ancestors()
            .filter(|ancestor| ancestor.extension() == Some(OsStr::new("zip")))
            .any(|ancestor| ancestor.is_file())
    }

    #[inline]
    #[instrument(name="yarn-archives.is_file", skip_all, fields(adaptator = "yarn-archives"))]
    fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        if let Some((archive, inner)) = split_virtual_path(path) {
            trace!("open archive {}", archive.display());
            let archive = File::open(archive)?;
            let archive = ZipArchive::new(archive)?;

            Ok(archive.index_for_path(inner).is_some())
        } else {
            Ok(false)
        }
    }
}

// Utils
fn split_virtual_path(path: &Path) -> Option<(&Path, &Path)> {
    let archive = path.ancestors()
        .filter(|ancestor| ancestor.extension() == Some(OsStr::new("zip")))
        .find(|ancestor| ancestor.is_file())?;

    Some((archive, path.strip_prefix(archive).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_supported_should_recognize_yarn_virtual_paths() {
        assert!(YarnArchives.is_supported(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));

        assert!(!YarnArchives.is_supported(Path::new("assets/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        assert!(YarnArchives.is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap());

        assert!(!YarnArchives.is_file(Path::new("assets/yarn-archive.zip/node_modules")).unwrap());
        assert!(!YarnArchives.is_file(Path::new("assets/yarn-archive.zip/do-not-exists")).unwrap());
    }
}