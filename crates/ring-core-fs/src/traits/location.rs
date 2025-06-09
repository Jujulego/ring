use std::path::PathBuf;
use crate::FsError;

pub trait Location {
    type Reader: std::io::Read;
    
    fn read(self) -> Result<Self::Reader, FsError>;
}

impl Location for PathBuf {
    type Reader = std::fs::File;
    
    #[inline]
    fn read(self) -> Result<Self::Reader, FsError> {
        std::fs::File::open(self).map_err(FsError::from)
    }
}