use crate::traits::{Location, MaybeLocationMetadata};
use crate::FsError;
use std::path::Path;

pub trait FilesystemMiddleware: MaybeLocationMetadata {
    /// Resolves given path and check if it points to anything
    fn maybe_locate_path(&self, path: &Path) -> Option<Result<Box<dyn Location + '_>, FsError>>;
}