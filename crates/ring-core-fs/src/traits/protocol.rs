use crate::traits::{Location, LocationMetadata};
use crate::FsError;
use std::path::Path;

pub type LocationIterator<'a> = Box<dyn Iterator<Item = Result<Box<dyn Location>, FsError>> + 'a>;

pub trait FilesystemProtocol: LocationMetadata {
    /// Resolves given path and check if it points to anything
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location + '_>, FsError>;

    /// Returns an iterator over path child locations
    fn list_content(&self, path: &Path) -> Result<LocationIterator<'_>, FsError>;
}
