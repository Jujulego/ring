use crate::traits::{AbsReader, Filesystem, MaybeFileMetadata, MaybeFilesystem};
use crate::Error;
use ring_core_utils::{Pool, PoolRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{debug, instrument, trace, warn};
use zip::ZipArchive;

/// Allow access to file stored in zip archives.
/// Supports yarn virtual paths.
#[derive(Debug)]
pub struct ZipMiddleware<F: Filesystem> {
    filesystem: Rc<F>,
    archives: RefCell<HashMap<PathBuf, Pool<ZipArchive<F::File>>>>
}

impl<F: Filesystem> ZipMiddleware<F> {
    #[inline]
    pub fn new(filesystem: Rc<F>) -> Self {
        Self {
            filesystem,
            archives: RefCell::new(HashMap::new()),
        }
    }
}

impl<F> ZipMiddleware<F>
where F: Filesystem,
      F::File: Read + Seek
{
    pub fn open_archive(&self, path: &Path) -> Result<PoolRef<ZipArchive<F::File>>, Error> {
        let mut archives = self.archives.borrow_mut();
        let pool = archives.entry(std::path::absolute(path)?).or_default();

        pool.try_borrow_or_build(|| {
            trace!("open archive {}", path.display());
            let archive = self.filesystem.open(path)?;
            Ok(ZipArchive::new(archive)?)
        })
    }
}

impl<F> MaybeFileMetadata for ZipMiddleware<F>
where F: Filesystem,
      F::File: Read + Seek
{
    #[instrument(name = "archives.is_dir", skip_all, fields(adaptator = "archives"))]
    fn maybe_is_dir(&self, path: &Path) -> Option<bool> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                return Some(false);
            }
        };

        let mut inner_path = zip::unstable::path_to_string(inner_path).to_string();
        inner_path += "/";

        Some(archive.file_names().any(|name| name.starts_with(&inner_path)))
    }

    #[instrument(name = "archives.is_file", skip_all, fields(adaptator = "archives"))]
    fn maybe_is_file(&self, path: &Path) -> Option<bool> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                return Some(false);
            }
        };

        Some(archive.index_for_path(inner_path).is_some())
    }
}

impl<F> MaybeFilesystem for ZipMiddleware<F>
where F: Filesystem,
      F::File: Read + Seek
{
    type File = ZippedFile<F::File>;

    #[instrument(name="archives.open", skip_all, fields(adaptator = "archives"))]
    fn open(&self, path: &Path) -> Option<Result<Self::File, Error>> {
        let path = parse_yarn_virtual_path(path);
        let (archive_path, inner_path) = split_archive_path(&path)?;

        let archive = match self.open_archive(archive_path) {
            Ok(archive) => archive,
            Err(err) => {
                warn!("unable to open archive {}", archive_path.display());
                debug!("error caused by: {err}");
                return Some(Err(err));
            }
        };

        let Some(index) = archive.index_for_path(inner_path) else {
            return Some(Err(Error::NotFound("File not found inside archive")));
        };

        Some(Ok(ZippedFile::new(archive, index)))
    }
}

/// Zipped file
pub struct ZippedFile<F> {
    archive: PoolRef<ZipArchive<F>>,
    file_index: usize,
}

impl<F> ZippedFile<F> {
    pub fn new(archive: PoolRef<ZipArchive<F>>, file_index: usize) -> Self {
        Self {
            archive,
            file_index,
        }
    }
}

impl<F: Read + Seek> AbsReader for ZippedFile<F> {
    fn abs_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self.archive.by_index(self.file_index).unwrap())
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
    use crate::filesystem::LocalFilesystem;

    #[test]
    fn is_dir_should_detect_directory_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalFilesystem));

        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/node_modules")), Some(true));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_dir(Path::new("assets")), None);
    }

    #[test]
    fn is_file_should_detect_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalFilesystem));

        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")), Some(true));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/node_modules")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/yarn-archive.zip/do-not-exists")), Some(false));
        assert_eq!(zip_middleware.maybe_is_file(Path::new("assets/foo.txt")), None);
    }

    #[test]
    fn open_should_allow_read_file_in_archive() {
        let zip_middleware = ZipMiddleware::new(Rc::new(LocalFilesystem));
        let mut file = zip_middleware.open(Path::new("assets/yarn-archive.zip/node_modules/foo.txt")).unwrap().unwrap();

        assert_eq!(std::io::read_to_string(file.abs_reader()).unwrap(), String::from("bar"));

        assert!(zip_middleware.open(Path::new("assets/foo.txt")).is_none());
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