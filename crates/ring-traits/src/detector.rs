use std::fmt::Debug;
use ring_utils::OptionalResult;
use std::path::Path;

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

/// Default implementation of Detector::detect_from 
#[macro_export]
macro_rules! detect_from {
    ($detector:ident, $path:ident) => {{
        use ring_utils::OptionalResult::{Found, Empty};
        let path = if $path.is_file() { $path.parent().unwrap() } else { $path };

        path.ancestors()
            .map(|ancestor| $detector.detect_at(ancestor))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }};
}

pub trait Detector : Debug {
    type Item;

    /// Search item at given path
    fn detect_at(&self, path: &Path) -> OptionalResult<Self::Item>;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &Path) -> OptionalResult<Self::Item> {
        detect_from!(self, path)
    }
}

pub trait DetectAs<T> : Debug {
    fn detect_at_as(&self, path: &Path) -> OptionalResult<T>;

    fn detect_from_as(&self, path: &Path) -> OptionalResult<T>;
}

impl<D : Detector> DetectAs<D::Item> for D {
    #[inline]
    fn detect_at_as(&self, path: &Path) -> OptionalResult<D::Item> {
        self.detect_at(path)
    }

    #[inline]
    fn detect_from_as(&self, path: &Path) -> OptionalResult<D::Item> {
        self.detect_from(path)
    }
}