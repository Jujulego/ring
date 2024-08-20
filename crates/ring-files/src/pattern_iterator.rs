use std::iter::FusedIterator;
use std::path::{Path, PathBuf};

pub trait PatternIterator : Iterator {
    fn relative_to(self, base: &Path) -> RelativePatterns<Self>
    where
        Self: Sized,
        Self::Item: AsRef<Path>
    {
        RelativePatterns::new(self, base)
    }
}

impl<I: Iterator> PatternIterator for I {}

pub struct RelativePatterns<I: Iterator>
where
    I::Item: AsRef<Path>
{
    iter: I,
    base: PathBuf,
}

impl<I: Iterator> RelativePatterns<I>
where
    I::Item: AsRef<Path>
{
    fn new(iter: I, base: &Path) -> RelativePatterns<I> {
        RelativePatterns {
            iter,
            base: dunce::simplified(base).to_path_buf(),
        }
    }

    #[inline]
    fn prepend_pattern(&self, pattern: I::Item) -> String {
        self.base.join(&pattern).to_str().unwrap_or_default().to_string()
    }
}

impl<I: Iterator> Iterator for RelativePatterns<I>
where
    I::Item: AsRef<Path>
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|pattern| self.prepend_pattern(pattern))
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for RelativePatterns<I>
where
    I::Item: AsRef<Path>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|pattern| self.prepend_pattern(pattern))
    }
}

impl<I: FusedIterator> FusedIterator for RelativePatterns<I>
where
    I::Item: AsRef<Path>
{}