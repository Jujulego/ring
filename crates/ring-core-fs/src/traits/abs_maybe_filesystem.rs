use crate::traits::{AbsReader, MaybeLocationMetadata, MaybeFilesystem};
use crate::FsError;
use std::path::Path;

pub trait AbsMaybeFilesystem: MaybeLocationMetadata {
    /// Opens given file
    #[inline]
    #[allow(unused_variables)]
    fn abs_maybe_open(&self, path: &Path) -> Option<Result<Box<dyn AbsReader + '_>, FsError>> {
        None
    }
}

impl<M> AbsMaybeFilesystem for M
where
    M: MaybeFilesystem,
    M::File: AbsReader
{
    #[inline]
    fn abs_maybe_open(&self, path: &Path) -> Option<Result<Box<dyn AbsReader + '_>, FsError>> {
        self.open(path)
            .map(|res| res.map(|x| Box::new(x) as Box<dyn AbsReader>))
    }
}