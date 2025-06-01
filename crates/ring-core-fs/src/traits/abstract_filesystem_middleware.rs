use crate::traits::FilesystemMiddleware;
use crate::Error;
use std::path::Path;

pub trait AbstractFilesystemMiddleware {
    /// Tests if given path exists and is a directory
    #[inline]
    fn is_dir(&self, _path: &Path) -> Option<bool> {
        None
    }

    /// Tests if given path exists and is a file
    #[inline]
    fn is_file(&self, _path: &Path) -> Option<bool> {
        None
    }

    /// Opens given file
    #[inline]
    fn open(&self, _path: &Path) -> Option<Result<Box<dyn std::io::Read + '_>, Error>> {
        None
    }
}

impl<M> AbstractFilesystemMiddleware for M
where
    M: FilesystemMiddleware,
    M::File: std::io::Read
{
    fn is_dir(&self, path: &Path) -> Option<bool> {
        self.is_dir(path)
    }

    fn is_file(&self, path: &Path) -> Option<bool> {
        self.is_file(path)
    }

    fn open(&self, path: &Path) -> Option<Result<Box<dyn std::io::Read + '_>, Error>> {
        self.open(path)
            .map(|res| res.map(|x| Box::new(x) as Box<dyn std::io::Read + '_>))
    }
}