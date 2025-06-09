use std::path::Path;
use crate::FsError;
use crate::traits::fs_location::FsLocation;

pub trait FsMiddleware {
    type Location: FsLocation;

    /// Resolves given path and check if it points to anything
    fn maybe_locate_path(&self, path: &Path) -> Option<Result<Self::Location, FsError>>;
}