use crate::traits::{Location, FsProtocol, FsReader, MaybeLocationMetadata};
use crate::{FsError, LocationType};
use ring_core_utils::{Pool, PoolRef};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
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

/// Zipped location
pub struct ZippedLocation<F> {
    archive: PoolRef<ZipArchive<F>>,
    file_index: Option<usize>,
}

impl<F> ZippedLocation<F> {
    pub fn new_directory(archive: PoolRef<ZipArchive<F>>) -> Self {
        Self {
            archive,
            file_index: None,
        }
    }

    pub fn new_file(archive: PoolRef<ZipArchive<F>>, file_index: usize) -> Self {
        Self {
            archive,
            file_index: Some(file_index),
        }
    }
}

impl<'a, F> Location for &'a mut ZippedLocation<F>
where F: std::io::Read + std::io::Seek,
{
    type Reader = ZipFile<'a, F>;

    fn read(self) -> Result<Self::Reader, FsError> {
        if let Some(file_index) = self.file_index {
            self.archive.by_index(file_index).map_err(FsError::from)
        } else {
            Err(FsError::NotAFile("Zipped location is not a file"))
        }
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
