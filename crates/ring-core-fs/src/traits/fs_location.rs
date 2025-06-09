use std::path::PathBuf;
use crate::FsError;

pub trait FsLocation {
    type Reader: std::io::Read;
    
    fn read(self) -> Result<Self::Reader, FsError>;
}

impl FsLocation for PathBuf {
    type Reader = std::fs::File;
    
    #[inline]
    fn read(self) -> Result<Self::Reader, FsError> {
        std::fs::File::open(self).map_err(FsError::from)
    }
}