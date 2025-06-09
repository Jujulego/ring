use std::path::Path;
use crate::FsError;
use crate::traits::fs_location::FsLocation;

pub trait FsProtocol {
    type Location: FsLocation;

    /// Resolves given path and check if it points to anything
    fn locate_path(&self, path: &Path) -> Result<Self::Location, FsError>;
}

pub type FsReader<P> = <<P as FsProtocol>::Location as FsLocation>::Reader;