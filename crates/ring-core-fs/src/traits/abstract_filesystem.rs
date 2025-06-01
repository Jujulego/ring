use crate::traits::Filesystem;
use crate::Error;
use std::path::Path;

pub trait AbstractFilesystem {
    /// Tests if given path exists and is a directory
    fn is_dir(&self, path: &Path) -> bool;

    /// Tests if given path exists and is a file
    fn is_file(&self, path: &Path) -> bool;

    /// Opens given file
    fn open(&self, path: &Path) -> Result<Box<dyn std::io::Read + '_>, Error>;
}

impl<F> AbstractFilesystem for F
where F: Filesystem,
      F::File: std::io::Read
{
    #[inline]
    fn is_dir(&self, path: &Path) -> bool {
        self.is_dir(path)
    }

    #[inline]
    fn is_file(&self, path: &Path) -> bool {
        self.is_file(path)
    }

    #[inline]
    fn open(&self, path: &Path) -> Result<Box<dyn std::io::Read + '_>, Error> {
        self.open(path)
            .map(|f| Box::new(f) as Box<dyn std::io::Read>)
    }
}