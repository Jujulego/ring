use anyhow::anyhow;
use std::iter::FusedIterator;
use std::path::Path;
use std::rc::Rc;
use ring_traits::DetectAs;
use ring_utils::{Normalize, NormalizedPath, NormalizedPathBuf, OptionalResult};

pub trait PatternIterator : Iterator {
    /// Uses given detector on each emitted normalized path
    #[inline]
    fn detect_at<T>(self, detector: Rc<dyn DetectAs<T>>) -> DetectedAt<Self, T>
    where
        Self: Sized,
        Self::Item: AsRef<NormalizedPath>
    {
        DetectedAt::new(self, detector)
    }

    /// Uses the glob crate to search files matching each emitted pattern
    #[cfg(feature = "glob")]
    #[inline]
    fn glob_search(self) -> GlobSearch<Self>
    where
        Self: Sized,
        Self::Item: AsRef<NormalizedPath>
    {
        GlobSearch::new(self)
    }

    /// Prepends each patterns with given base
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```
    /// use std::path::Path;
    /// use ring_files::PatternIterator;
    /// use ring_utils::Normalize;
    ///
    /// let base = Path::new("/example").normalize();
    /// let patterns = vec!["crates/*", "scripts"];
    /// let prepended = patterns.iter().resolve(&base).collect::<Vec<_>>();
    ///
    /// assert_eq!(prepended, &["/example/crates/*", "/example/scripts"]);
    /// ```
    /// 
    /// It does not prepend absolute patterns:
    /// 
    /// ```
    /// use std::path::Path;
    /// use ring_files::PatternIterator;
    /// use ring_utils::Normalize;
    ///
    /// let base = Path::new("/example").normalize();
    /// let patterns = vec!["/crates/*", "/scripts"];
    /// let prepended = patterns.iter().resolve(&base).collect::<Vec<_>>();
    ///
    /// assert_eq!(prepended, &["/crates/*", "/scripts"]);
    /// ```
    #[inline]
    fn resolve(self, base: &NormalizedPath) -> ResolvedPatterns<Self>
    where
        Self: Sized,
        Self::Item: AsRef<Path>
    {
        ResolvedPatterns::new(self, base)
    }
}

impl<I: Iterator> PatternIterator for I {}

pub struct DetectedAt<I: Iterator, T>
where
    I::Item: AsRef<NormalizedPath>
{
    iter: I,
    detector: Rc<dyn DetectAs<T>>
}

impl<I: Iterator, T> DetectedAt<I, T>
where
    I::Item: AsRef<NormalizedPath>
{
    fn new(iter: I, detector: Rc<dyn DetectAs<T>>) -> DetectedAt<I, T> {
        DetectedAt { iter, detector }
    }
}

impl<I: Iterator, T> Iterator for DetectedAt<I, T>
where
    I::Item: AsRef<NormalizedPath>
{
    type Item = anyhow::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let path = self.iter.next()?;

            match self.detector.detect_at_as(path.as_ref()) {
                OptionalResult::Found(item) => return Some(Ok(item)),
                OptionalResult::Fail(err) => return Some(Err(err)),
                OptionalResult::Empty => continue
            }
        }
    }
}

impl<I: FusedIterator, T> FusedIterator for DetectedAt<I, T>
where
    I::Item: AsRef<NormalizedPath>
{}

#[cfg(feature = "glob")]
pub struct GlobSearch<I: Iterator>
where
    I::Item: AsRef<NormalizedPath>
{
    iter: I,
    paths: Option<glob::Paths>,
}

#[cfg(feature = "glob")]
impl<I: Iterator> GlobSearch<I>
where
    I::Item: AsRef<NormalizedPath>
{
    fn new(iter: I) -> GlobSearch<I> {
        GlobSearch {
            iter,
            paths: None,
        }
    }
}

#[cfg(feature = "glob")]
impl<I: Iterator> Iterator for GlobSearch<I>
where
    I::Item: AsRef<NormalizedPath>
{
    type Item = anyhow::Result<NormalizedPathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(paths) = &mut self.paths {
                match paths.next() {
                    Some(Ok(path)) => break Some(Ok(path.normalize())),
                    Some(Err(err)) => {
                        let context = format!("Unable to access {}", err.path().display());
                        break Some(Err(anyhow!(err.into_error()).context(context)))
                    },
                    None => {
                        self.paths = None;
                    }
                }
            }

            if let Some(pattern) = self.iter.next() {
                match glob::glob(pattern.as_ref().as_os_str().to_str()?) {
                    Ok(paths) => {
                        self.paths = Some(paths);
                    }
                    Err(err) => break Some(Err(
                        anyhow!(err).context(format!("Error while parsing pattern {}", pattern.as_ref().display()))
                    )),
                }
            } else {
                break None;
            }
        }
    }
}

#[cfg(feature = "glob")]
impl<I: FusedIterator> FusedIterator for GlobSearch<I>
where
    I::Item: AsRef<NormalizedPath>
{}

pub struct ResolvedPatterns<'a, I: Iterator>
where
    I::Item: AsRef<Path>
{
    iter: I,
    base: &'a NormalizedPath,
}

impl<'a, I: Iterator> ResolvedPatterns<'a, I>
where
    I::Item: AsRef<Path>
{
    fn new(iter: I, base: &'a NormalizedPath) -> ResolvedPatterns<'a, I> {
        ResolvedPatterns { iter, base }
    }

    #[inline]
    fn prepend_pattern(&self, pattern: I::Item) -> NormalizedPathBuf {
        pattern.as_ref().resolve(self.base)
    }
}

impl<'a, I: Iterator> Iterator for ResolvedPatterns<'a, I>
where
    I::Item: AsRef<Path>
{
    type Item = NormalizedPathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|pattern| self.prepend_pattern(pattern))
    }
}

impl<'a, I: DoubleEndedIterator> DoubleEndedIterator for ResolvedPatterns<'a, I>
where
    I::Item: AsRef<Path>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|pattern| self.prepend_pattern(pattern))
    }
}

impl<'a, I: FusedIterator> FusedIterator for ResolvedPatterns<'a, I>
where
    I::Item: AsRef<Path>
{}