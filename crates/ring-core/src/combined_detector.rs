use std::iter::FusedIterator;
use std::path::Path;
use std::rc::Rc;
use ring_traits::DetectAs;
use ring_utils::OptionalResult;

#[derive(Debug)]
pub struct CombinedDetector<T> {
    detectors: Vec<Rc<dyn DetectAs<T>>>,
}

impl<T> CombinedDetector<T> {
    pub fn new(detectors: Vec<Rc<dyn DetectAs<T>>>) -> CombinedDetector<T> {
        CombinedDetector { detectors }
    }

    pub fn detect_from<'a>(&'a self, path: &'a Path) -> Iter<'a, T> {
        Iter {
            detectors: self.detectors.as_slice(),
            strategy: DetectStrategy::From(path)
        }
    }

    pub fn detect_at<'a>(&'a self, path: &'a Path) -> Iter<'a, T> {
        Iter {
            detectors: self.detectors.as_slice(),
            strategy: DetectStrategy::At(path)
        }
    }
}

#[derive(Debug)]
enum DetectStrategy<'a> {
    From(&'a Path),
    At(&'a Path)
}

impl<'a> DetectStrategy<'a> {
    fn apply<T>(&self, detector: &Rc<dyn DetectAs<T>>) -> OptionalResult<T> {
        match self {
            DetectStrategy::From(path) => detector.detect_from_as(path),
            DetectStrategy::At(path) => detector.detect_at_as(path),
        }
    }
}

#[derive(Debug)]
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