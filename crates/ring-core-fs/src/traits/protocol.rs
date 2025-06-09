use std::path::Path;
use crate::FsError;
use crate::traits::location::Location;

pub trait FsProtocol {
    type Location: Location;

    /// Resolves given path and check if it points to anything
    fn locate_path(&self, path: &Path) -> Result<Self::Location, FsError>;
}

pub type FsReader<P> = <<P as FsProtocol>::Location as Location>::Reader;