use std::path::Path;

pub trait Detector {
    type Item;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> DetectorResult<Self::Item>;
}

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_result_into_detector_result() {
        assert_eq!(DetectorResult::<&str, ()>::from(Ok("test")),  DetectorResult::Found("test"));
        assert_eq!(DetectorResult::<(), &str>::from(Err("test")), DetectorResult::Err("test"));
    }

    #[test]
    fn it_should_convert_option_into_detector_result() {
        assert_eq!(DetectorResult::<&str, ()>::from(Some("test")), DetectorResult::Found("test"));
        assert_eq!(DetectorResult::<(),   ()>::from(None),         DetectorResult::None);
    }

    #[test]
    fn it_should_convert_detector_result_into_result() {
        assert_eq!(DetectorResult::<&str, ()>::Found("test").into_result(), Ok(Some("test")));
        assert_eq!(DetectorResult::<(), &str>::Err("test").into_result(),   Err("test"));
        assert_eq!(DetectorResult::<(),   ()>::None.into_result(),          Ok(None));
    }

    #[test]
    fn it_should_convert_detector_result_into_option() {
        assert_eq!(DetectorResult::<&str, ()>::Found("test").into_option(), Some(Ok("test")));
        assert_eq!(DetectorResult::<(), &str>::Err("test").into_option(),   Some(Err("test")));
        assert_eq!(DetectorResult::<(),   ()>::None.into_option(),          None);
    }
}