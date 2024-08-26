use ring_traits::DetectAs;
use ring_utils::{NormalizedPath, OptionalResult};
use std::iter::FusedIterator;
use std::rc::Rc;

#[derive(Default)]
pub struct CombinedDetector<T> {
    detectors: Vec<Rc<dyn DetectAs<T>>>,
}

impl<T> CombinedDetector<T> {
    pub fn new(detectors: Vec<Rc<dyn DetectAs<T>>>) -> CombinedDetector<T> {
        CombinedDetector { detectors }
    }

    pub fn detect_from<'a, P: AsRef<NormalizedPath>>(&'a self, path: &'a P) -> Iter<'a, T> {
        Iter {
            detectors: self.detectors.as_slice(),
            strategy: DetectStrategy::From(path.as_ref()),
        }
    }

    pub fn detect_at<'a, P: AsRef<NormalizedPath>>(&'a self, path: &'a P) -> Iter<'a, T> {
        Iter {
            detectors: self.detectors.as_slice(),
            strategy: DetectStrategy::At(path.as_ref()),
        }
    }
}

#[derive(Debug)]
enum DetectStrategy<'a> {
    From(&'a NormalizedPath),
    At(&'a NormalizedPath),
}

impl<'a> DetectStrategy<'a> {
    fn apply<T>(&self, detector: &Rc<dyn DetectAs<T>>) -> OptionalResult<T> {
        // TODO: pass a normalized path to detector
        match self {
            DetectStrategy::From(path) => detector.detect_from_as(path.as_ref()),
            DetectStrategy::At(path) => detector.detect_at_as(path.as_ref()),
        }
    }
}

pub struct Iter<'a, T> {
    detectors: &'a [Rc<dyn DetectAs<T>>],
    strategy: DetectStrategy<'a>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = anyhow::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let detector = self.detectors.first()?;
            self.detectors = &self.detectors[1..];

            if let result @ Some(_) = self.strategy.apply(detector).into() {
                break result;
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            let detector = self.detectors.last()?;
            self.detectors = &self.detectors[..self.detectors.len() - 1];

            if let result @ Some(_) = self.strategy.apply(detector).into() {
                break result;
            }
        }
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    use anyhow::anyhow;
    use mockall::mock;
    use ring_traits::DetectAs;
    use ring_utils::Normalize;
    use ring_utils::OptionalResult::{Empty, Fail, Found};

    mock!(
        Detector {}

        impl DetectAs<&'static str> for Detector {
            fn detect_at_as(&self, path: &Path) -> OptionalResult<&'static str>;
            fn detect_from_as(&self, path: &Path) -> OptionalResult<&'static str>;
        }
    );

    #[test]
    fn it_should_use_all_given_detectors_with_at_strategy() {
        let detector_a = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Found("a"));
            detector
        });

        let detector_b = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Found("b"));
            detector
        });

        let detector_empty = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Empty);
            detector
        });

        let detector_fail = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Fail(anyhow!("Failed !")));
            detector
        });

        let combined = CombinedDetector::new(vec![
            detector_a, detector_b, detector_empty, detector_fail
        ]);

        let results: Vec<_> = combined.detect_at(&Path::new("/test").normalize()).collect();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().ok(), Some(&"a"));
        assert_eq!(results[1].as_ref().ok(), Some(&"b"));
        assert!(results[2].is_err());
    }

    #[test]
    fn it_should_use_all_given_detectors_with_from_strategy() {
        let detector_a = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_from_as().returning(|_| Found("a"));
            detector
        });

        let detector_b = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_from_as().returning(|_| Found("b"));
            detector
        });

        let detector_empty = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_from_as().returning(|_| Empty);
            detector
        });

        let detector_fail = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_from_as().returning(|_| Fail(anyhow!("Failed !")));
            detector
        });

        let combined = CombinedDetector::new(vec![
            detector_a, detector_b, detector_empty, detector_fail
        ]);

        let results: Vec<_> = combined.detect_from(&Path::new("/test").normalize()).collect();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].as_ref().ok(), Some(&"a"));
        assert_eq!(results[1].as_ref().ok(), Some(&"b"));
        assert!(results[2].is_err());
    }

    #[test]
    fn it_should_return_reversible_iterator() {
        let detector_a = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Found("a"));
            detector
        });

        let detector_b = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Found("b"));
            detector
        });

        let detector_empty = Rc::new({
            let mut detector = MockDetector::new();
            detector.expect_detect_at_as().returning(|_| Empty);
            detector
        });

        let combined = CombinedDetector::new(vec![detector_a, detector_b, detector_empty]);

        let results: Vec<_> = combined.detect_at(&Path::new("/test").normalize()).rev().collect();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].as_ref().ok(), Some(&"b"));
        assert_eq!(results[1].as_ref().ok(), Some(&"a"));
    }
}
