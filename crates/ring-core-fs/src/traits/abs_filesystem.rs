use crate::traits::{AbsReader, FileMetadata, Filesystem};
use crate::Error;
use std::path::Path;

/// Filesystem interaction (returning abstract objects)
pub trait AbsFilesystem: FileMetadata {
    /// Opens given file
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, Error>;
}

impl<F> AbsFilesystem for F
where F: Filesystem,
      F::File: AbsReader
{
    #[inline]
    fn abs_open(&self, path: &Path) -> Result<Box<dyn AbsReader + '_>, Error> {
        self.open(path)
            .map(|f| Box::new(f) as Box<dyn AbsReader>)
    }
}