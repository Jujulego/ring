use crate::traits::{AbsReader, LocationMetadata, Filesystem};
use crate::FsError;
use std::path::Path;

/// Filesystem interaction (returning abstract objects)
pub trait AbsFilesystem: LocationMetadata {
    /// Opens given file
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, FsError>;
}

impl<F> AbsFilesystem for F
where F: Filesystem,
      F::File: AbsReader
{
    #[inline]
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, FsError> {
        self.open(path)
            .map(|f| Box::new(f) as Box<dyn AbsReader>)
    }
}