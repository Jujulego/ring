use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::path::{Component, Components, Path, PathBuf, PrefixComponent, MAIN_SEPARATOR_STR};

////////////////////////////////////////////////////////////////////////////////
// Normalized component
////////////////////////////////////////////////////////////////////////////////

/// A single normalized component of a path.
///
/// See [`Component`] for more details
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use ring_utils::{Normalize, NormalizedComponent};
///
/// let path = Path::new("/tmp/foo/bar.txt").normalize();
/// let components = path.components().collect::<Vec<_>>();
/// assert_eq!(&components, &[
///   NormalizedComponent::RootDir,
///   NormalizedComponent::Normal("tmp".as_ref()),
///   NormalizedComponent::Normal("foo".as_ref()),
///   NormalizedComponent::Normal("bar.txt".as_ref()),
/// ]);
/// ```
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum NormalizedComponent<'a> {
    Prefix(PrefixComponent<'a>),
    RootDir,
    Normal(&'a OsStr),
}

impl<'a> NormalizedComponent<'a> {
    /// Extracts the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/tmp/foo/bar.txt").normalize();
    /// let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
    /// assert_eq!(&components, &["/", "tmp", "foo", "bar.txt"]);
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn as_os_str(self) -> &'a OsStr {
        match self {
            NormalizedComponent::Prefix(p) => p.as_os_str(),
            NormalizedComponent::RootDir => OsStr::new(MAIN_SEPARATOR_STR),
            NormalizedComponent::Normal(path) => path
        }
    }
}

impl AsRef<OsStr> for NormalizedComponent<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<NormalizedPath> for NormalizedComponent<'_> {
    #[inline]
    fn as_ref(&self) -> &NormalizedPath {
        NormalizedPath::new(self.as_os_str())
    }
}

impl<'a> From<Component<'a>> for NormalizedComponent<'a> {
    /// Converts a [`Component`] into a [`NormalizedComponent`].
    /// Panics if a [`Component::CurDir`] or [`Component::ParentDir`] is given.
    fn from(value: Component<'a>) -> NormalizedComponent<'a> {
        match value {
            Component::Prefix(prefix) => NormalizedComponent::Prefix(prefix),
            Component::RootDir => NormalizedComponent::RootDir,
            Component::CurDir => panic!("cannot convert CurDir Component to NormalizedComponent"),
            Component::ParentDir => panic!("cannot convert ParentDir Component to NormalizedComponent"),
            Component::Normal(c) => NormalizedComponent::Normal(c),
        }
    }
}

impl<'a> PartialEq<Component<'a>> for NormalizedComponent<'a> {
    /// Compare a NormalizedComponent to a Component
    fn eq(&self, other: &Component<'a>) -> bool {
        match (self, other) {
            (NormalizedComponent::Prefix(a), Component::Prefix(b)) => a == b,
            (NormalizedComponent::RootDir, Component::RootDir) => true,
            (NormalizedComponent::Normal(a), Component::Normal(b)) => a == b,
            (_, _) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalized components iterator
////////////////////////////////////////////////////////////////////////////////

/// An iterator over the [`NormalizedComponent`]s of a [`NormalizedPath`].
///
/// See [`Components`] for more details
#[derive(Clone, Debug)]
pub struct NormalizedComponents<'a> {
    inner: Components<'a>,
}

impl<'a> NormalizedComponents<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    #[must_use]
    pub fn as_path(&self) -> &'a NormalizedPath {
        NormalizedPath::new(self.inner.as_path())
    }
}

impl AsRef<NormalizedPath> for NormalizedComponents<'_> {
    fn as_ref(&self) -> &NormalizedPath {
        self.as_path()
    }
}

impl AsRef<OsStr> for NormalizedComponents<'_> {
    fn as_ref(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

impl AsRef<Path> for NormalizedComponents<'_> {
    fn as_ref(&self) -> &Path {
        self.as_path().as_path()
    }
}

impl<'a> Iterator for NormalizedComponents<'a> {
    type Item = NormalizedComponent<'a>;

    fn next(&mut self) -> Option<NormalizedComponent<'a>> {
        self.inner.next().map(NormalizedComponent::from)
    }
}

impl<'a> DoubleEndedIterator for NormalizedComponents<'a> {
    fn next_back(&mut self) -> Option<NormalizedComponent<'a>> {
        self.inner.next_back().map(NormalizedComponent::from)
    }
}

impl FusedIterator for NormalizedComponents<'_> {}

impl<'a> PartialEq for NormalizedComponents<'a> {
    fn eq(&self, other: &NormalizedComponents<'a>) -> bool {
        self.inner == other.inner
    }
}

impl Eq for NormalizedComponents<'_> {}

impl<'a> PartialEq<Components<'a>> for NormalizedComponents<'a> {
    fn eq(&self, other: &Components<'a>) -> bool {
        self.inner == *other
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalized Ancestors
////////////////////////////////////////////////////////////////////////////////

/// An iterator over [`NormalizedPath`] and its ancestors.
#[derive(Clone, Copy, Debug)]
pub struct NormalizedAncestors<'a> {
    next: Option<&'a NormalizedPath>,
}

impl<'a> Iterator for NormalizedAncestors<'a> {
    type Item = &'a NormalizedPath;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(NormalizedPath::parent);
        next
    }
}

impl FusedIterator for NormalizedAncestors<'_> {}

////////////////////////////////////////////////////////////////////////////////
// Normalized Path
////////////////////////////////////////////////////////////////////////////////

/// Wrapper of [`Path`] representing path without any '.' or '..' components
///
/// # Examples
///
/// ```
/// use ring_utils::Normalize;
/// use std::path::Path;
///
/// let path = Path::new("/example/bar/baz/../foo").normalize();
///
/// assert_eq!(path, Path::new("/example/bar/foo"));
/// ```
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(not(doc), repr(transparent))]
pub struct NormalizedPath {
    inner: Path,
}

impl NormalizedPath {
    fn new<P: AsRef<Path> + ?Sized>(p: &P) -> &NormalizedPath {
        unsafe { &*(p.as_ref() as *const Path as *const NormalizedPath) }
    }

    /// Yields the underlying [`Path`]
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &Path {
        &self.inner
    }

    /// Yields the underlying [`OsStr`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/example/../foo.txt").normalize();
    /// assert_eq!(path.as_os_str(), std::ffi::OsStr::new("/foo.txt"));
    /// ```
    #[must_use]
    #[inline]
    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
    }

    /// Returns an iterator over [`NormalizedPath`] and its ancestors.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use ring_utils::{Normalize, NormalizedComponent};
    ///
    /// let path = Path::new("/example/foo/baz/../bar").normalize();
    /// let mut components = path.ancestors();
    ///
    /// assert_eq!(components.next(), Some(Path::new("/example/foo/bar").normalize().as_ref()));
    /// assert_eq!(components.next(), Some(Path::new("/example/foo").normalize().as_ref()));
    /// assert_eq!(components.next(), Some(Path::new("/example").normalize().as_ref()));
    /// assert_eq!(components.next(), Some(Path::new("/").normalize().as_ref()));
    /// assert_eq!(components.next(), None);
    /// ```
    pub fn ancestors(&self) -> NormalizedAncestors<'_> {
        NormalizedAncestors { next: Some(self) }
    }

    /// Returns an iterator over [`NormalizedComponents`] of path
    ///
    /// # Examples
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use ring_utils::{Normalize, NormalizedComponent};
    ///
    /// let path = Path::new("/example/foo/baz/../bar").normalize();
    /// let mut components = path.components();
    ///
    /// assert_eq!(components.next(), Some(NormalizedComponent::RootDir));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("example"))));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("foo"))));
    /// assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("bar"))));
    /// assert_eq!(components.next(), None);
    /// ```
    #[inline]
    pub fn components(&self) -> NormalizedComponents {
        NormalizedComponents { inner: self.inner.components() }
    }

    /// Returns path without its final component, if there is one.
    ///
    /// See [`Path::parent`] for more details
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/foo/baz/../bar").normalize();
    /// let parent = path.parent().unwrap();
    /// assert_eq!(parent, Path::new("/foo"));
    ///
    /// let grand_parent = parent.parent().unwrap();
    /// assert_eq!(grand_parent, Path::new("/"));
    /// assert_eq!(grand_parent.parent(), None);
    /// ```
    #[inline]
    pub fn parent(&self) -> Option<&NormalizedPath> {
        self.inner.parent().map(NormalizedPath::new)
    }
}

impl AsRef<OsStr> for &NormalizedPath {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<Path> for &NormalizedPath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl PartialEq<&Path> for &NormalizedPath {
    fn eq(&self, other: &&Path) -> bool {
        self.components().eq(other.components())
    }
}

impl PartialEq<PathBuf> for &NormalizedPath {
    fn eq(&self, other: &PathBuf) -> bool {
        self.components().eq(other.components())
    }
}

impl PartialEq<NormalizedPathBuf> for &NormalizedPath {
    fn eq(&self, other: &NormalizedPathBuf) -> bool {
        self.components().eq(other.components())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalized Path Buffer
////////////////////////////////////////////////////////////////////////////////

/// Wrapper of [`PathBuf`] representing path without any '.' or '..' components
///
/// # Examples
///
/// ```
/// use ring_utils::Normalize;
/// use std::path::Path;
///
/// let path = Path::new("/example/bar/baz/../foo").normalize();
///
/// assert_eq!(path, Path::new("/example/bar/foo"));
/// ```
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NormalizedPathBuf {
    inner: PathBuf,
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

    /// Removes the last component of the path
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

impl AsRef<NormalizedPath> for NormalizedPathBuf {
    fn as_ref(&self) -> &NormalizedPath {
        self.deref()
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

impl Deref for NormalizedPathBuf {
    type Target = NormalizedPath;

    fn deref(&self) -> &Self::Target {
        NormalizedPath::new(self.inner.as_path())
    }
}

impl Borrow<NormalizedPath> for NormalizedPathBuf {
    fn borrow(&self) -> &NormalizedPath {
        self.deref()
    }
}

impl From<&NormalizedPath> for NormalizedPathBuf {
    fn from(value: &NormalizedPath) -> Self {
        NormalizedPathBuf { inner: PathBuf::from(value.as_path()) }
    }
}

impl<P: AsRef<Path>> FromIterator<P> for NormalizedPathBuf {
    fn from_iter<T: IntoIterator<Item=P>>(iter: T) -> Self {
        let mut buf = NormalizedPathBuf::new();
        buf.extend(iter);
        buf
    }
}

impl<P: AsRef<Path>> Extend<P> for NormalizedPathBuf {
    fn extend<I: IntoIterator<Item = P>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |p| self._push(p.as_ref()));
    }
}

impl<P: ?Sized + AsRef<Path>> PartialEq<&P> for NormalizedPathBuf {
    fn eq(&self, other: &&P) -> bool {
        self.components().eq(other.as_ref().components())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalize trait
////////////////////////////////////////////////////////////////////////////////

pub trait Normalize : AsRef<Path> {
    fn normalize(&self) -> NormalizedPathBuf;

    fn resolve(&self, base: &NormalizedPath) -> NormalizedPathBuf {
        let mut ret = NormalizedPathBuf::from(base);
        ret._push(self.as_ref());
        ret
    }
}

impl Normalize for NormalizedPathBuf {
    fn normalize(&self) -> NormalizedPathBuf {
        self.clone()
    }
}

impl Normalize for Path {
    fn normalize(&self) -> NormalizedPathBuf {
        let mut components = self.components().peekable();
        let mut inner = match components.peek().cloned() {
            Some(c @ Component::Prefix(..)) => {
                components.next();
                PathBuf::from(c.as_os_str())
            }
            Some(Component::RootDir) | None => {
                PathBuf::new()
            }
            _ => panic!("normalized path must start with either a RootDir or a Prefix")
        };

        components.for_each(|c| normalized_push(&mut inner, c));

        NormalizedPathBuf { inner }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_component_into_normalized_component() {
        assert_eq!(NormalizedComponent::from(Component::RootDir), NormalizedComponent::RootDir);
        assert_eq!(NormalizedComponent::from(Component::Normal("test".as_ref())), NormalizedComponent::Normal("test".as_ref()));
    }

    #[test]
    #[should_panic(expected = "cannot convert CurDir Component to NormalizedComponent")]
    fn it_should_panic_when_trying_to_convert_cur_dir() {
        let _ = NormalizedComponent::from(Component::CurDir);
    }

    #[test]
    #[should_panic(expected = "cannot convert ParentDir Component to NormalizedComponent")]
    fn it_should_panic_when_trying_to_convert_parent_dir() {
        let _ = NormalizedComponent::from(Component::ParentDir);
    }

    #[test]
    fn it_should_eq_normalized_component_with_component() {
        assert_eq!(NormalizedComponent::RootDir, Component::RootDir);
        assert_ne!(NormalizedComponent::RootDir, Component::CurDir);
        assert_ne!(NormalizedComponent::RootDir, Component::ParentDir);
        assert_ne!(NormalizedComponent::RootDir, Component::Normal("test".as_ref()));

        assert_ne!(NormalizedComponent::Normal("test".as_ref()), Component::RootDir);
        assert_ne!(NormalizedComponent::Normal("test".as_ref()), Component::CurDir);
        assert_ne!(NormalizedComponent::Normal("test".as_ref()), Component::ParentDir);
        assert_eq!(NormalizedComponent::Normal("test".as_ref()), Component::Normal("test".as_ref()));
    }
}
