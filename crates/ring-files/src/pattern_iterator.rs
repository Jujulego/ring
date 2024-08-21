use anyhow::anyhow;
use std::iter::FusedIterator;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_traits::DetectAs;
use ring_utils::OptionalResult;

pub trait PatternIterator : Iterator {
    /// Uses given detector on each emitted canonical path
    #[inline]
    fn detect_at<T>(self, detector: Rc<dyn DetectAs<T>>) -> DetectedAt<Self, T>
    where
        Self: Sized,
        Self::Item: AsRef<Path>
    {
        DetectedAt::new(self, detector)
    }

    /// Uses the glob crate to search files matching each emitted pattern
    #[cfg(feature = "glob")]
    #[inline]
    fn glob_search(self) -> GlobSearch<Self>
    where
        Self: Sized,
        Self::Item: AsRef<str>
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
    /// use ring_files::PatternIterator;
    /// 
    /// // Note: this example does work on Windows
    /// let patterns = vec!["crates/*", "scripts"];
    /// let prepended = patterns.iter().relative_to("/example").collect::<Vec<String>>();
    /// 
    /// assert_eq!(prepended, &["/example/crates/*", "/example/scripts"]);
    /// ```
    /// 
    /// It does not prepend absolute patterns:
    /// 
    /// ```
    /// use ring_files::PatternIterator;
    /// 
    /// let patterns = vec!["/crates/*", "/scripts"];
    /// let prepended = patterns.iter().relative_to("/example").collect::<Vec<String>>();
    /// 
    /// assert_eq!(prepended, &["/crates/*", "/scripts"]);
    /// ```
    #[inline]
    fn relative_to<P: AsRef<Path>>(self, base: P) -> RelativePatterns<Self>
    where
        Self: Sized,
        Self::Item: AsRef<Path>
    {
        RelativePatterns::new(self, base.as_ref())
    }
}

impl<I: Iterator> PatternIterator for I {}

pub struct DetectedAt<I: Iterator, T>
where
    I::Item: AsRef<Path>
{
    iter: I,
    detector: Rc<dyn DetectAs<T>>
}

impl<I: Iterator, T> DetectedAt<I, T>
where
    I::Item: AsRef<Path>
{
    fn new(iter: I, detector: Rc<dyn DetectAs<T>>) -> DetectedAt<I, T> {
        DetectedAt { iter, detector }
    }
}

impl<I: Iterator, T> Iterator for DetectedAt<I, T>
where
    I::Item: AsRef<Path>
{
    type Item = anyhow::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let path = self.iter.next()?;
            let path = match path.as_ref().canonicalize() {
                Ok(p) => { p }
                Err(err) => {
                    return Some(Err(anyhow!(err).context(format!("Unable to access {}", path.as_ref().display()))))
                }
            };

            match self.detector.detect_at_as(&path) {
                OptionalResult::Found(item) => return Some(Ok(item)),
                OptionalResult::Fail(err) => return Some(Err(err)),
                OptionalResult::Empty => continue
            }
        }
    }
}

impl<I: FusedIterator, T> FusedIterator for DetectedAt<I, T>
where
    I::Item: AsRef<Path>
{}

#[cfg(feature = "glob")]
pub struct GlobSearch<I: Iterator>
where
    I::Item: AsRef<str>
{
    iter: I,
    paths: Option<glob::Paths>,
}

#[cfg(feature = "glob")]
impl<I: Iterator> GlobSearch<I>
where
    I::Item: AsRef<str>
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
    I::Item: AsRef<str>
{
    type Item = anyhow::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(paths) = &mut self.paths {
                match paths.next() {
                    Some(Ok(path)) => break Some(Ok(path)),
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
                match glob::glob(pattern.as_ref()) {
                    Ok(paths) => {
                        self.paths = Some(paths);
                    }
                    Err(err) => break Some(Err(
                        anyhow!(err).context(format!("Error while parsing pattern {}", pattern.as_ref()))
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
    I::Item: AsRef<str>
{}

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