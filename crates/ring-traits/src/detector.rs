use std::path::Path;
use crate::optional_result::OptionalResult;

pub trait Detector {
    type Item;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item>;
}
