use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::{Component, Components, Path, PathBuf, PrefixComponent};

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum NormalizedComponent<'a> {
    Prefix(PrefixComponent<'a>),
    RootDir,
    Normal(&'a OsStr),
}

impl<'a> From<Component<'a>> for NormalizedComponent<'a> {
    fn from(value: Component<'a>) -> Self {
        match value {
            Component::Prefix(prefix) => NormalizedComponent::Prefix(prefix),
            Component::RootDir => NormalizedComponent::RootDir,
            Component::Normal(c) => NormalizedComponent::Normal(c),
            c => panic!("un-normalized component {:?}", c),
        }
    }
}

impl<'a, 'b> PartialEq<Component<'a>> for NormalizedComponent<'b> {
    fn eq(&self, other: &Component<'a>) -> bool {
        match (self, other) {
            (NormalizedComponent::Prefix(a), Component::Prefix(b)) => a == b,
            (NormalizedComponent::RootDir, Component::RootDir) => true,
            (NormalizedComponent::Normal(a), Component::Normal(b)) => a == b,
            (_, _) => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NormalizedComponents<'a> {
    inner: Components<'a>,
}

impl<'a> Iterator for NormalizedComponents<'a> {
    type Item = NormalizedComponent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(NormalizedComponent::from)
    }
}

impl<'a> DoubleEndedIterator for NormalizedComponents<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(NormalizedComponent::from)
    }
}

/// Wrapper of [`PathBuf`] representing path without any '.' or '..' components
///
/// # Examples
///
/// ```
/// use ring_utils::NormalizedPathBuf;
/// use std::path::Path;
///
/// let path = NormalizedPathBuf::from("/example/bar/baz/../foo");
///
/// assert_eq!(path, Path::new("/example/bar/foo"));
/// ```
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NormalizedPathBuf {
    inner: PathBuf,
}

impl NormalizedPathBuf {
    /// Allocates a new NormalizedPathBuf
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_utils::NormalizedPathBuf;
    ///
    /// let path = NormalizedPathBuf::new();
    /// ```
    #[inline]
    pub fn new() -> NormalizedPathBuf {
        NormalizedPathBuf { inner: PathBuf::new() }
    }

    /// Returns an iterator over [`NormalizedComponents`] of path
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use ring_utils::{NormalizedComponent, NormalizedPathBuf};
    ///
    /// let path = NormalizedPathBuf::from("/example/bar/baz/../foo");
    /// let mut components = path.components();
    ///
    /// assert_eq!(components.next(), Some(NormalizedComponent::RootDir));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("example"))));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("bar"))));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("foo"))));
    /// assert_eq!(components.next(), None);
    /// ```
    #[inline]
    pub fn components(&self) -> NormalizedComponents {
        NormalizedComponents { inner: self.inner.components() }
    }

    #[inline]
    pub fn pop(&mut self) {
        self.inner.pop();
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    #[inline]
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        self._push(path.as_ref())
    }

    fn _push(&mut self, path: &Path) {
        if path.is_absolute() {
            self.inner.clear();
        }

        path.components().for_each(|c| normalized_push(&mut self.inner, c));
    }
}

fn normalized_push(path: &mut PathBuf, component: Component) {
    match component {
        Component::Prefix(..) => unreachable!(),
        Component::RootDir => {
            path.push(component.as_os_str());
        }
        Component::CurDir => {}
        Component::ParentDir => {
            path.pop();
        }
        Component::Normal(c) => {
            path.push(c);
        }
    }
}

impl AsRef<OsStr> for NormalizedPathBuf {
    fn as_ref(&self) -> &OsStr {
        self.inner.as_ref()
    }
}

impl AsRef<Path> for NormalizedPathBuf {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}

impl<P: AsRef<Path>> Extend<P> for NormalizedPathBuf {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |p| self._push(p.as_ref()));
    }

    #[inline]
    fn extend_one(&mut self, item: P) {
        self._push(item.as_ref());
    }
}

impl<T: ?Sized + AsRef<Path>> From<&T> for NormalizedPathBuf {
    /// Converts a borrowed [`Path`] to a [`NormalizedPathBuf`].
    ///
    /// Allocates a [`NormalizedPathBuf`] and copies the data into it.
    fn from(value: &T) -> Self {
        value.as_ref().normalize()
    }
}

impl<P: AsRef<Path>> FromIterator<P> for NormalizedPathBuf {
    fn from_iter<T: IntoIterator<Item=P>>(iter: T) -> Self {
        let mut buf = NormalizedPathBuf::new();
        buf.extend(iter);
        buf
    }
}

impl PartialEq<&Path> for NormalizedPathBuf {
    fn eq(&self, other: &&Path) -> bool {
        self.components().eq(other.components())
    }
}

pub trait Normalize {
    fn normalize(&self) -> NormalizedPathBuf;
}

impl Normalize for Path {
    fn normalize(&self) -> NormalizedPathBuf {
        let mut components = self.components().peekable();
        let mut inner = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
            components.next();
            PathBuf::from(c.as_os_str())
        } else {
            PathBuf::new()
        };

        components.for_each(|c| normalized_push(&mut inner, c));

        NormalizedPathBuf { inner }
    }
}