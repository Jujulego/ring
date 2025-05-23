use crate::PathAdaptator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use tracing::{instrument, trace};
use zip::ZipArchive;

/// Access files in yarn archives. Supports yarn virtual paths.
pub struct YarnArchives {
    archives: RefCell<HashMap<PathBuf, Weak<RefCell<ZipArchive<File>>>>>
}

impl YarnArchives {
    #[inline]
    pub fn new() -> Self {
        YarnArchives {
            archives: RefCell::new(HashMap::new())
        }
    }

    fn open_archive(&self, path: &Path) -> anyhow::Result<Rc<RefCell<ZipArchive<File>>>> {
        if let Some(archive) = self.archives.borrow().get(path).and_then(Weak::upgrade) {
            Ok(archive.clone())
        } else {
            trace!("open archive {}", path.display());
            let archive = File::open(path)?;
            let archive = ZipArchive::new(archive)?;
            let archive = Rc::new(RefCell::new(archive));

            self.archives.borrow_mut().insert(path.to_owned(), Rc::downgrade(&archive));

            Ok(archive)
        }
    }
}

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
        let (archive_path, inner_path) = split_virtual_path(path).unwrap();
        let archive = self.open_archive(archive_path)?;

        Ok(archive.borrow().index_for_path(inner_path).is_some())
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
        let archives = YarnArchives::new();

        assert!(archives.is_supported(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
        assert!(!archives.is_supported(Path::new("assets/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        let archives = YarnArchives::new();

        assert!(archives.is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap());

        assert!(!archives.is_file(Path::new("assets/yarn-archive.zip/node_modules")).unwrap());
        assert!(!archives.is_file(Path::new("assets/yarn-archive.zip/do-not-exists")).unwrap());
    }
}