use crate::PathAdaptator;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use tracing::{debug, instrument, trace, warn};
use zip::ZipArchive;

/// Access files in archives. Supports yarn virtual paths.
pub struct ArchivesAdaptator {
    archives: RefCell<HashMap<PathBuf, Weak<RefCell<ZipArchive<File>>>>>
}

impl ArchivesAdaptator {
    #[inline]
    pub fn new() -> Self {
        ArchivesAdaptator {
            archives: RefCell::new(HashMap::new())
        }
    }

    fn open_archive(&self, path: &Path) -> anyhow::Result<Rc<RefCell<ZipArchive<File>>>> {
        let path = path.canonicalize()?;

        if let Some(archive) = self.archives.borrow().get(&path).and_then(Weak::upgrade) {
            Ok(archive.clone())
        } else {
            trace!("open archive {}", path.display());
            let archive = File::open(&path)?;
            let archive = ZipArchive::new(archive)?;
            let archive = Rc::new(RefCell::new(archive));

            self.archives.borrow_mut().insert(path, Rc::downgrade(&archive));

            Ok(archive)
        }
    }
}

impl Default for ArchivesAdaptator {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl PathAdaptator for ArchivesAdaptator {
    #[inline]
    fn is_supported(&self, path: &Path) -> bool {
        path.ancestors()
            .any(|ancestor| ancestor.extension().map(OsStr::to_string_lossy) == Some("zip".into()))
    }

    #[inline]
    #[instrument(name="archives.is_dir", skip_all, fields(adaptator = "archives"))]
    fn is_dir(&self, path: &Path) -> bool {
        let (archive_path, inner_path) = split_archive_path(path).unwrap();

        match self.open_archive(&archive_path) {
            Ok(archive) => {
                let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();
                inner_path += "/";

                archive.borrow()
                    .file_names()
                    .any(|name| name.starts_with(&inner_path))
            }
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                false
            }
        }
    }

    #[inline]
    #[instrument(name="archives.is_file", skip_all, fields(adaptator = "archives"))]
    fn is_file(&self, path: &Path) -> bool {
        let (archive_path, inner_path) = split_archive_path(path).unwrap();

        match self.open_archive(&archive_path) {
            Ok(archive) => {
                archive.borrow()
                    .index_for_path(inner_path).is_some()
            }
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                false
            }
        }
    }
}

// Utils
fn split_archive_path(path: &Path) -> Option<(PathBuf, &Path)> {
    let archive = path.ancestors()
        .find(|ancestor| ancestor.extension() == Some(OsStr::new("zip")))?;

    let inner = path.strip_prefix(archive).ok()?;
    let archive = parse_yarn_virtual_path(archive);

    Some((archive, inner))
}

fn parse_yarn_virtual_path(path: &Path) -> PathBuf {
    let Some(mut base) = path.ancestors()
        .find(|ancestor| ancestor.file_name().map(|n| n.to_string_lossy()) == Some("__virtual__".into()))
        .and_then(Path::parent)
    else {
        return path.to_path_buf();
    };

    let (rest, back_count) = {
        let mut components = path.strip_prefix(base).unwrap().components();
        components.next(); // ignore "__virtual__"
        components.next(); // ignore package archive name
        let back = components.next()
            .map(|c| c.as_os_str().to_string_lossy())
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap();

        (components.as_path(), back)
    };

    for _ in 0..back_count {
        base = base.parent().unwrap();
    }

    base.join(rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_supported_should_recognize_yarn_virtual_paths() {
        let archives = ArchivesAdaptator::new();

        assert!(archives.is_supported(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
        assert!(!archives.is_supported(Path::new("assets/foo.txt")));
    }

    #[test]
    fn is_file_should_detect_directory_in_archive() {
        let archives = ArchivesAdaptator::new();

        assert!(archives.is_dir(Path::new("assets/yarn-archive.zip/node_modules")));

        assert!(!archives.is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));
        assert!(!archives.is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")));
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        let archives = ArchivesAdaptator::new();

        assert!(archives.is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")));

        assert!(!archives.is_file(Path::new("assets/yarn-archive.zip/node_modules")));
        assert!(!archives.is_file(Path::new("assets/yarn-archive.zip/do-not-exists")));
    }
}