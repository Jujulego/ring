use crate::error::Error;
use crate::pool::{Pool, PoolRef};
use crate::{FileWrapper, PathAdaptator};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{debug, instrument, trace, warn};
use zip::ZipArchive;

/// Access files in archives. Supports yarn virtual paths.
pub struct ArchivesAdaptator {
    archives: RefCell<HashMap<PathBuf, Pool<ZipArchive<File>>>>
}

impl ArchivesAdaptator {
    #[inline]
    pub fn new() -> Self {
        ArchivesAdaptator {
            archives: RefCell::new(HashMap::new())
        }
    }

    fn _open_archive(&self, path: &Path) -> Result<PoolRef<ZipArchive<File>>, Error> {
        let path = std::path::absolute(path)?;

        let mut archives = self.archives.borrow_mut();
        let pool = archives.entry(path.clone()).or_default();

        pool.try_borrow_or_build(|| {
            trace!("open archive {}", path.display());
            let archive = File::open(path)?;
            Ok(ZipArchive::new(archive)?)
        })
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
    #[instrument(name="archives.is_dir", skip_all, fields(adaptator = "archives"))]
    fn is_dir(&self, path: &Path) -> bool {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path).unwrap();

        match self._open_archive(archive_path) {
            Ok(archive) => {
                let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();
                inner_path += "/";

                archive.file_names()
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
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path).unwrap();

        match self._open_archive(archive_path) {
            Ok(archive) => {
                archive.index_for_path(inner_path).is_some()
            }
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                false
            }
        }
    }

    #[inline]
    fn is_supported(&self, path: &Path) -> bool {
        path.ancestors()
            .any(|ancestor| ancestor.extension().map(OsStr::to_string_lossy) == Some("zip".into()))
    }

    #[inline]
    #[instrument(name="archives.open", skip_all, fields(adaptator = "archives"))]
    fn open(&self, path: &Path) -> Result<Box<dyn FileWrapper>, Error> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path).unwrap();

        let archive = self._open_archive(archive_path)?;
        let Some(index) = archive.index_for_path(inner_path) else {
            return Err(Error::NotFound("File not found inside archive"))
        };

        Ok(Box::new(ZippedFile { archive, file_index: index }))
    }
}

pub struct ZippedFile {
    archive: PoolRef<ZipArchive<File>>,
    file_index: usize,
}

impl FileWrapper for ZippedFile {
    #[instrument(name="archives.reader", skip_all, fields(adaptator = "archives"))]
    fn reader(&mut self) -> Result<Box<dyn Read + '_>, Error> {
        trace!("decompress {}", self.archive.name_for_index(self.file_index).unwrap());
        Ok(Box::new(self.archive.by_index(self.file_index)?))
    }
}

// Utils
/// Splits an archive path in two, the path to the archive and the path in the archive to the file
fn split_archive_path(path: &Path) -> Option<(&Path, &Path)> {
    let archive = path.ancestors()
        .find(|ancestor| ancestor.extension() == Some(OsStr::new("zip")))?;

    let inner = path.strip_prefix(archive).ok()?;

    Some((archive, inner))
}

/// Parses yarn virtual paths to a valid archive path.
pub fn parse_yarn_virtual_path(path: &Path) -> PathBuf {
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

    #[cfg(target_os = "linux")]
    #[test]
    fn test_parse_yarn_virtual_path() {
        let virtual_path = Path::new("/home/toto/project/.yarn/__virtual__/cool-virtual-hash/2/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js");

        assert_eq!(
            parse_yarn_virtual_path(virtual_path),
            Path::new("/home/toto/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js")
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_parse_yarn_virtual_path() {
        let virtual_path = Path::new(r"C:\Users\toto\project\.yarn\__virtual__\cool-virtual-hash\2\AppData\Local\Yarn\Berry\cache\cool-hash.zip\node_modules\cool\cool.js");

        assert_eq!(
            parse_yarn_virtual_path(virtual_path),
            Path::new(r"C:\Users\toto\AppData\Local\Yarn\Berry\cache\cool-hash.zip\node_modules\cool\cool.js")
        );
    }
}