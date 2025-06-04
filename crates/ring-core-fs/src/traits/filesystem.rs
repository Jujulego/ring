use crate::traits::file_metadata::FileMetadata;
use crate::Error;
use std::path::Path;

pub trait Filesystem: FileMetadata {
    type File;

    /// Opens given file
    fn open(&self, path: &Path) -> Result<Self::File, Error>;
}