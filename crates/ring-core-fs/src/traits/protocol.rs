use crate::traits::{Location, LocationMetadata};
use crate::FsError;
use std::path::Path;

pub trait FilesystemProtocol: LocationMetadata {
    /// Resolves given path and check if it points to anything
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location + '_>, FsError>;
}
