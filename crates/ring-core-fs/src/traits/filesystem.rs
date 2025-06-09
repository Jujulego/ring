use crate::traits::location_metadata::LocationMetadata;
use crate::Error;
use std::path::Path;

pub trait Filesystem: LocationMetadata {
    type File;

    /// Opens given file
    fn open(&self, path: &Path) -> Result<Self::File, Error>;
}