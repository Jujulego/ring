use crate::traits::{FsMiddleware, FsProtocol, FsReader, Location, MaybeLocationMetadata};
use crate::{FsError, LocationType};
use ring_core_utils::{Pool, PoolRef, ReadSeek};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, trace, warn};
use zip::read::ZipFile;
use zip::ZipArchive;

/// Allow access to file stored in zip archives.
/// Supports yarn virtual paths.
pub struct ZipMiddleware<P: FsProtocol> {
    protocol: Rc<P>,
    archives: RefCell<HashMap<PathBuf, Pool<ZipArchive<FsReader<P>>>>>
}
impl<P: FsProtocol> ZipMiddleware<P> {
    #[inline]
    pub fn new(protocol: Rc<P>) -> Self {
        Self {
            protocol,
            archives: RefCell::new(HashMap::new()),
        }
    }
}

impl<P> ZipMiddleware<P>
where P: FsProtocol,
      FsReader<P>: std::io::Read + std::io::Seek
{
    pub fn open_archive(&self, path: &Path) -> Result<PoolRef<ZipArchive<FsReader<P>>>, FsError> {
        let mut archives = self.archives.borrow_mut();

        archives.entry(std::path::absolute(path)?).or_default()
            .try_borrow_or_build(|| {
                trace!("open archive {}", path.display());
                let mut archive = self.protocol.locate_path(path)?;
                let file = archive.read()?;

                Ok(ZipArchive::new(file)?)
            })
            .inspect_err(|err| {
                warn!("unable to open archive {}", path.display());
                debug!("error caused by: {err}");
            })
    }
}

impl<P> MaybeLocationMetadata for ZipMiddleware<P>
where P: FsProtocol,
      FsReader<P>: std::io::Read + std::io::Seek
{
    fn maybe_location_type(&self, path: &Path) -> Option<Result<LocationType, FsError>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();

        if archive.index_for_name(&inner_path).is_some() {
            return Some(Ok(LocationType::File));
        }

        inner_path += "/";

        if archive.file_names().any(|name| name.starts_with(&inner_path)) {
            Some(Ok(LocationType::Directory))
        } else {
            Some(Err(FsError::NotFound("Location not found in archive")))
        }
    }

    fn maybe_is_symlink(&self, path: &Path) -> Option<bool> {
        let path = parse_yarn_virtual_path(path);
        split_archive_path(&path).map(|_| false)
    }
}

impl<P> FsMiddleware for ZipMiddleware<P>
where P: FsProtocol,
      FsReader<P>: std::io::Read + std::io::Seek
{
    type Location = ZippedLocation<FsReader<P>>;

    fn maybe_locate_path(&self, path: &Path) -> Option<Result<Self::Location, FsError>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();

        if let Some(file_index) = archive.index_for_name(&inner_path) {
            return Some(Ok(ZippedLocation::new_file(archive, file_index)));
        }

        inner_path += "/";

        if archive.file_names().any(|name| name.starts_with(&inner_path)) {
            Some(Ok(ZippedLocation::new_directory(archive)))
        } else {
            Some(Err(FsError::NotFound("Location not found in archive")))
        }
    }
}

/// Zipped location
pub struct ZippedLocation {
    archive: PoolRef<ZipArchive<Box<dyn ReadSeek>>>,
    file_index: Option<usize>,
}

impl ZippedLocation {
    pub fn new_directory(archive: PoolRef<ZipArchive<Box<dyn ReadSeek>>>) -> Self {
        Self {
            archive,
            file_index: None,
        }
    }

    pub fn new_file(archive: PoolRef<ZipArchive<Box<dyn ReadSeek>>>, file_index: usize) -> Self {
        Self {
            archive,
            file_index: Some(file_index),
        }
    }
}

impl Location for ZippedLocation {
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        if let Some(file_index) = self.file_index {
            self.archive.by_index(file_index)
                .map(|file| Box::new(file) as _)
                .map_err(FsError::from)
        } else {
            Err(FsError::NotAFile("Zipped location is not a file"))
        }
    }

    fn read_seek(&mut self) -> Result<Box<dyn ReadSeek + '_>, FsError> {
        if let Some(file_index) = self.file_index {
            self.archive.by_index_seek(file_index)
                .map(|file| Box::new(file) as _)
                .map_err(FsError::from)
        } else {
            Err(FsError::NotAFile("Zipped location is not a file"))
        }    }
}

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
    use crate::protocols::LocalProtocol;

    #[test]
    fn location_type_should_detect_files_and_directories_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalProtocol));

        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(Ok(LocationType::File))));
        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/node_modules")), Some(Ok(LocationType::Directory))));
        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(Err(FsError::NotFound(_)))));
        assert!(zip_middleware.maybe_location_type(Path::new("assets")).is_none());
    }

    #[test]
    fn is_dir_should_detect_directory_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalProtocol));

        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules")), Some(true));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets")), None);
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalProtocol));

        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(true));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/foo.txt")), None);
    }

    #[test]
    fn is_symlink_should_detect_nothing_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalProtocol));

        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/node_modules")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/foo.txt")), None);
    }

    #[test]
    fn open_should_allow_read_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalProtocol));
        let mut file = zip_middleware.maybe_locate_path(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap().unwrap();

        assert_eq!(std::io::read_to_string(file.read().unwrap()).unwrap(), String::from("bar"));

        assert!(zip_middleware.maybe_locate_path(Path::new("assets/foo.txt")).is_none());
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

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_parse_yarn_virtual_path() {
        let virtual_path = Path::new("/home/toto/project/.yarn/__virtual__/cool-virtual-hash/2/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js");

        assert_eq!(
            parse_yarn_virtual_path(virtual_path),
            Path::new("/home/toto/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js")
        );
    }
}
