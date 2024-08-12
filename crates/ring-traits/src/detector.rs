use ring_utils::OptionalResult;
use std::path::Path;

pub trait Detector {
    type Item;

    /// Search item at given path
    fn detect_at(&self, path: &Path) -> OptionalResult<Self::Item>;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item>;
}

pub trait DetectAs<T> {
    fn detect_at_as(&self, path: &Path) -> OptionalResult<T>;

    fn detect_from_as(&self, path: &Path) -> OptionalResult<T>;
}

#[macro_export]
macro_rules! detect_as {
    ($base:ident, $item:ty) => {
        impl DetectAs<$item> for $base {
            fn detect_at_as(&self, path: &Path) -> OptionalResult<$item> {
                self.detect_from(path)
                    .map(|prj| prj as $item)
            }

            fn detect_from_as(&self, path: &Path) -> OptionalResult<$item> {
                self.detect_from(path)
                    .map(|prj| prj as $item)
            }
        }
    };
}
