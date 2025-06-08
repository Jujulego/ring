use crate::traits::MaybeLocationMetadata;
use crate::Error;
use std::path::Path;

pub trait MaybeFilesystem: MaybeLocationMetadata {
    type File;

    /// Opens given file
    #[inline]
    #[allow(unused_variables)]
    fn open(&self, path: &Path) -> Option<Result<Self::File, Error>> {
        None
    }
}