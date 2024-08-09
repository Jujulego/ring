use std::path::Path;

pub trait Detector {
    type Item;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> DetectorResult<Self::Item>;
}

#[derive(Debug)]
pub enum DetectorResult<T, E = anyhow::Error> {
    Found(T),
    Err(E),
    None,
}

impl<T, E> From<Result<T, E>> for DetectorResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(data) => DetectorResult::Found(data),
            Err(err) => DetectorResult::Err(err),
        }
    }
}

impl<T> From<Option<T>> for DetectorResult<T, ()> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(data) => DetectorResult::Found(data),
            None => DetectorResult::None,
        }
    }
}

impl<T, E> DetectorResult<T, E> {
    pub fn into_option(self) -> Option<Result<T, E>> {
        match self {
            DetectorResult::Found(data) => Some(Ok(data)),
            DetectorResult::Err(err) => Some(Err(err)),
            DetectorResult::None => None,
        }
    }

    pub fn into_result(self) -> Result<Option<T>, E> {
        match self {
            DetectorResult::Found(data) => Ok(Some(data)),
            DetectorResult::Err(err) => Err(err),
            DetectorResult::None => Ok(None),
        }
    }
}
