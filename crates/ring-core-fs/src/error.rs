use std::io;
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("NotFound: {0}")]
    NotFound(&'static str),

    #[error("IoError: {0}")]
    IoError(#[source] io::Error),
    #[error("ZipError: {0}")]
    ZipError(#[source] ZipError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::NotFound {
            Error::NotFound("Path not found")
        } else {
            Error::IoError(err)
        }
    }
}

impl From<ZipError> for Error {
    fn from(value: ZipError) -> Self {
        match value {
            ZipError::Io(err) => err.into(),
            ZipError::FileNotFound => Error::NotFound("File not found in archive"),
            err => Error::ZipError(err)
        }
    }
}
