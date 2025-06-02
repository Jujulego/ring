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
