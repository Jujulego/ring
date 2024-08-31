use ring_utils::{NormalizedPath, OptionalResult};

#[macro_export]
macro_rules! detect_as {
    ($base:ident, $item:ty) => {
        impl DetectAs<$item> for $base {
            fn detect_at_as(&self, path: &ring_utils::NormalizedPath) -> OptionalResult<$item> {
                self.detect_at(path)
                    .map(|prj| prj as $item)
            }

            fn detect_from_as(&self, path: &ring_utils::NormalizedPath) -> OptionalResult<$item> {
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

        $path.ancestors()
            .map(|ancestor| $detector.detect_at(ancestor))
            .find(|res| matches!(res, Found(_)))
            .unwrap_or(Empty)
    }};
}

pub trait Detect {
    type Item;

    /// Search item at given path
    fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<Self::Item>;

    /// Search item from given path (and ancestors)
    fn detect_from(&self, path: &NormalizedPath) -> OptionalResult<Self::Item> {
        detect_from!(self, path)
    }
}

pub trait DetectAs<T> {
    fn detect_at_as(&self, path: &NormalizedPath) -> OptionalResult<T>;

    fn detect_from_as(&self, path: &NormalizedPath) -> OptionalResult<T>;
}

impl<D : Detect> DetectAs<D::Item> for D {
    #[inline]
    fn detect_at_as(&self, path: &NormalizedPath) -> OptionalResult<D::Item> {
        self.detect_at(path)
    }

    #[inline]
    fn detect_from_as(&self, path: &NormalizedPath) -> OptionalResult<D::Item> {
        self.detect_from(path)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use mockall::mock;
    use ring_utils::Normalize;
    use ring_utils::OptionalResult::{Empty, Found};
    use super::*;

    mock!(
        Detector {}
        impl Detect for Detector {
            type Item = String;

            fn detect_at(&self, path: &NormalizedPath) -> OptionalResult<String>;
        }
    );

    #[test]
    fn it_should_call_detect_at_using_all_path_ancestors() {
        let mut detector = MockDetector::new();
        detector.expect_detect_at()
            .times(3)
            .returning(|path| if path == "/test" { Found(String::from("test")) } else { Empty });

        let path = Path::new("/test/foo/bar").normalize();
        assert!(detector.detect_from(&path).is_found());
        
        detector.checkpoint();
    }
}