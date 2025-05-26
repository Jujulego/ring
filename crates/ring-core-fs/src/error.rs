use std::io;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum FsError {
    #[error("NotFound: {0}")]
    NotFound(&'static str),

    #[error("IoError: {0}")]
    IoError(#[source] io::Error),
    #[error("ZipError: {0}")]
    ZipError(#[source] ZipError),
}

impl From<io::Error> for FsError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            FsError::NotFound("Path not found")
        } else {
            FsError::IoError(err)
        }
    }
}

impl From<ZipError> for FsError {
    fn from(value: ZipError) -> Self {
        match value {
            ZipError::Io(err) => err.into(),
            ZipError::FileNotFound => FsError::NotFound("File not found in archive"),
            err => FsError::ZipError(err)
        }
    }
}
