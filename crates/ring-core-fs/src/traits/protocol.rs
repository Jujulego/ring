use std::path::Path;
use crate::FsError;
use crate::traits::location::Location;

pub trait FsProtocol {
    /// Resolves given path and check if it points to anything
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location + '_>, FsError>;
}
