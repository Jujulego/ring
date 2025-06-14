use crate::traits::{FilesystemMiddleware, Location, LocationIterator, MaybeLocationMetadata};
use crate::{FsError, LocationType};
use ring_core_utils::{Pool, PoolRef};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::OsStr;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, trace, warn};
use zip::ZipArchive;

pub trait ZipOrigin {
    type File: Read + Seek + 'static;
    
    fn open_zip(&self, path: &Path) -> Result<Self::File, FsError>;
}

/// Allow access to file stored in zip archives.
/// Supports yarn virtual paths.
pub struct ZipMiddleware<O: ZipOrigin> {
    origin: Rc<O>,
    archives: RefCell<HashMap<PathBuf, Pool<ZipArchive<O::File>>>>,
}

impl<O: ZipOrigin> ZipMiddleware<O> {
    #[inline]
    pub fn new(origin: Rc<O>) -> Self {
        Self {
            origin,
            archives: RefCell::new(HashMap::new()),
        }
    }
}

impl<O: ZipOrigin> ZipMiddleware<O> {
    pub fn open_archive(&self, path: &Path) -> Result<PoolRef<ZipArchive<O::File>>, FsError> {
        let mut archives = self.archives.borrow_mut();

        archives.entry(std::path::absolute(path)?).or_default()
            .try_borrow_or_build(|| {
                trace!(middleware = "zip", "open archive {}", path.display());
                let file = self.origin.open_zip(path)?;

                Ok(ZipArchive::new(file)?)
            })
            .inspect_err(move |err| {
                warn!(middleware = "zip", "Unable to open archive {}", path.display());
                debug!(middleware = "zip", "error caused by: {err}");
            })
    }
}

impl<O: ZipOrigin> MaybeLocationMetadata for ZipMiddleware<O> {
    fn maybe_location_type(&self, path: &Path) -> Option<Result<LocationType, FsError>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let mut archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();

        if let Ok(file) = archive.by_name(&inner_path) {
            let t = if file.is_dir() {
                LocationType::Directory
            } else if file.is_symlink() {
                LocationType::Symlink
            } else {
                LocationType::File
            };

            return Some(Ok(t));
        }

        inner_path += "/";

        if archive.file_names().any(|name| name.starts_with(&inner_path)) {
            Some(Ok(LocationType::Directory))
        } else {
            Some(Err(FsError::NotFound("Location not found in archive")))
        }
    }
}

impl<O: ZipOrigin> FilesystemMiddleware for ZipMiddleware<O> {
    fn maybe_locate_path(&self, path: &Path) -> Option<Result<Box<dyn Location + '_>, FsError>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();

        if archive.index_for_name(&inner_path).is_some() {
            let location = ZippedLocation::new(
                archive,
                archive_path.to_path_buf(),
                inner_path
            );

            return Some(Ok(Box::new(location)));
        }

        inner_path += "/";

        if archive.file_names().any(|name| name.starts_with(&inner_path)) {
            let location = ZippedLocation::new(
                archive,
                archive_path.to_path_buf(),
                inner_path
            );

            Some(Ok(Box::new(location)))
        } else {
            Some(Err(FsError::NotFound("Location not found in archive")))
        }
    }

    fn maybe_read_dir(&self, path: &Path) -> Option<Result<LocationIterator<'_>, FsError>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let base = inner_path.components()
            .filter_map(|component| component.as_os_str().to_str())
            .collect::<Vec<_>>();

        let children = archive.file_names()
            .map(|file| file
                .split('/')
                .take(base.len() + 1)
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
            )
            .filter(|file| file.len() > base.len() && file.starts_with(&base))
            .map(|file| file.join("/"))
            .collect::<HashSet<_>>();

        let iterator = ZippedIterator::new(self, archive_path.to_path_buf(), children.into_iter().collect());
        Some(Ok(Box::new(iterator) as _))
    }
}

/// Zipped location
pub struct ZippedLocation<F> {
    archive: PoolRef<ZipArchive<F>>,
    archive_path: PathBuf,
    inner_path: String,
}

impl<F> ZippedLocation<F> {
    pub fn new(archive: PoolRef<ZipArchive<F>>, archive_path: PathBuf, inner_path: String) -> Self {
        Self {
            archive,
            archive_path,
            inner_path,
        }
    }
}

impl<F: Read + Seek> Location for ZippedLocation<F> {
    #[inline]
    fn path(&self) -> PathBuf {
        self.archive_path.join(&self.inner_path)
    }
    
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        match self.archive.by_name(&self.inner_path) {
            Ok(file) if file.is_file() => Ok(Box::new(file)),
            Ok(_) => Err(FsError::NotAFile("Zipped location is not a file")),
            Err(err) => Err(err.into()),
        }
    }
}

/// Iterator on zipped location
pub struct ZippedIterator<'a, O: ZipOrigin> {
    zip_middleware: &'a ZipMiddleware<O>,
    archive_path: PathBuf,
    children: VecDeque<String>,
}

impl<'a, O: ZipOrigin> ZippedIterator<'a, O> {
    pub fn new(zip_middleware: &'a ZipMiddleware<O>, archive_path: PathBuf,children: VecDeque<String>) -> Self {
        Self {
            zip_middleware,
            archive_path,
            children
        }
    }
}

impl<'a, O: ZipOrigin> Iterator for ZippedIterator<'a, O> {
    type Item = Result<Box<dyn Location>, FsError>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.children.pop_front()?;
        let archive = match self.zip_middleware.open_archive(&self.archive_path) {
            Ok(archive) => archive,
            Err(err) => return Some(Err(err))
        };

        let location = ZippedLocation::new(archive, self.archive_path.clone(), next);
        Some(Ok(Box::new(location)))
    }
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
    use crate::protocols::FileProtocol;

    #[test]
    fn location_type_should_detect_files_and_directories_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(Ok(LocationType::File))));
        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/node_modules")), Some(Ok(LocationType::Directory))));
        assert!(matches!(zip_middleware.maybe_location_type(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(Err(FsError::NotFound(_)))));
        assert!(zip_middleware.maybe_location_type(Path::new("assets")).is_none());
    }

    #[test]
    fn is_dir_should_detect_directory_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules")), Some(true));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets")), None);
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(true));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/foo.txt")), None);
    }

    #[test]
    fn is_symlink_should_detect_nothing_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/node_modules")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_symlink(Path::new("assets/foo.txt")), None);
    }

    #[test]
    fn protocol_should_allow_read_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        // Existing file in an archive
        let mut file = zip_middleware.maybe_locate_path(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap().unwrap();

        assert_eq!(file.read_to_string().unwrap(), String::from("bar"));

        // Existing folder in an archive
        let mut file = zip_middleware.maybe_locate_path(Path::new("assets/yarn-archive.zip/node_modules")).unwrap().unwrap();

        assert!(matches!(file.read_to_string(), Err(FsError::NotAFile(_))));

        // Not existing path
        assert!(matches!(zip_middleware.maybe_locate_path(Path::new("assets/yarn-archive.zip/does-not-exists")).unwrap(), Err(FsError::NotFound(_))));

        // Existing file out of an archive
        assert!(zip_middleware.maybe_locate_path(Path::new("assets/foo.txt")).is_none());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn protocol_should_allow_to_read_directory() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        let mut locations = zip_middleware.maybe_read_dir(Path::new("assets/yarn-archive.zip/node_modules")).unwrap().unwrap()
            .map(|location| location.unwrap().path())
            .collect::<Vec<_>>();

        locations.sort();

        assert_eq!(locations, vec![
            PathBuf::from(r"assets/yarn-archive.zip\\node_modules/foo.txt"),
            PathBuf::from(r"assets/yarn-archive.zip\\node_modules/toto.txt")
        ]);
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn protocol_should_allow_to_read_directory() {
        let zip_middleware = ZipMiddleware::new(Rc::new(FileProtocol));

        let mut locations = zip_middleware.maybe_read_dir(Path::new("assets/yarn-archive.zip/node_modules")).unwrap().unwrap()
            .map(|location| location.unwrap().path())
            .collect::<Vec<_>>();

        locations.sort();

        assert_eq!(locations, vec![
            PathBuf::from(r"assets/yarn-archive.zip/node_modules/foo.txt"),
            PathBuf::from(r"assets/yarn-archive.zip/node_modules/toto.txt")
        ]);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_parse_yarn_virtual_path() {
        let virtual_path = Path::new(r"C:\Users\toto\project\.yarn\__virtual__\cool-virtual-hash\2\AppData\Local\Yarn\Berry\cache\cool-hash.zip\node_modules\cool\cool.js");

        assert_eq!(
            parse_yarn_virtual_path(virtual_path),
            Path::new(r"C:\Users\toto\AppData\Local\Yarn\Berry\cache\cool-hash.zip\node_modules\cool\cool.js")
        );
    }

    #[test]
    #[cfg(not(target_os = "windows"))]
    fn test_parse_yarn_virtual_path() {
        let virtual_path = Path::new("/home/toto/project/.yarn/__virtual__/cool-virtual-hash/2/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js");

        assert_eq!(
            parse_yarn_virtual_path(virtual_path),
            Path::new("/home/toto/.Yarn/Berry/cache/cool-hash.zip/node_modules/cool/cool.js")
        );
    }
}
