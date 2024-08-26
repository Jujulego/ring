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
    /// use std::path::{Path, MAIN_SEPARATOR_STR};
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/tmp/foo/bar.txt").normalize();
    /// let components: Vec<_> = path.components().map(|comp| comp.as_os_str()).collect();
    /// assert_eq!(&components, &[MAIN_SEPARATOR_STR, "tmp", "foo", "bar.txt"]);
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
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use ring_utils::Normalize;
///
/// let path = Path::new("/tmp/foo/bar.txt").normalize();
///
/// for component in path.components() {
///     println!("{component:?}");
/// }
/// ```
#[derive(Clone, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct NormalizedComponents<'a> {
    inner: Components<'a>,
}

impl<'a> NormalizedComponents<'a> {
    /// Extracts a slice corresponding to the portion of the path remaining for iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/tmp/foo/bar.txt").normalize();
    /// let mut components = path.components();
    /// components.next();
    /// components.next();
    ///
    /// assert_eq!(Path::new("foo/bar.txt"), components.as_path());
    /// ```
    #[must_use]
    #[inline]
    pub fn as_path(&self) -> &'a Path {
        self.inner.as_path()
    }
}

impl AsRef<OsStr> for NormalizedComponents<'_> {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        self.as_path().as_os_str()
    }
}

impl AsRef<Path> for NormalizedComponents<'_> {
    #[inline]
    fn as_ref(&self) -> &Path {
        self.as_path()
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
    #[inline]
    fn eq(&self, other: &NormalizedComponents<'a>) -> bool {
        self.inner == other.inner
    }
}

impl Eq for NormalizedComponents<'_> {}

impl<'a> PartialEq<Components<'a>> for NormalizedComponents<'a> {
    #[inline]
    fn eq(&self, other: &Components<'a>) -> bool {
        self.inner == *other
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalized Ancestors
////////////////////////////////////////////////////////////////////////////////

/// An iterator over [`NormalizedPath`] and its ancestors.
///
/// See [`Ancestors`] for more details.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use ring_utils::Normalize;
///
/// let path = Path::new("/foo/bar").normalize();
///
/// for ancestor in path.ancestors() {
///     println!("{}", ancestor.display());
/// }
/// ```
///
/// [`Ancestors`]: std::path::Ancestors
#[derive(Clone, Copy, Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/example/../foo.txt").normalize();
    /// assert_eq!(path.as_path(), Path::new("/foo.txt"));
    /// ```
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
    /// use std::ffi::OsStr;
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/example/../foo.txt").normalize();
    /// assert_eq!(path.as_os_str(), OsStr::new("/foo.txt"));
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

    /// Returns an object that implements [`Display`] for safely printing paths.
    ///
    /// For more details see [`Path::display`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/tmp/foo.rs").normalize();
    ///
    /// println!("{}", path.display());
    /// ```
    #[inline]
    pub fn display(&self) -> std::path::Display<'_> {
        self.inner.display()
    }

    /// Returns `true` if the path exists on disk and is pointing at a directory.
    ///
    /// See [`Path::is_dir`] for more details
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// assert_eq!(Path::new("/is_a_directory/").normalize().is_dir(), true);
    /// assert_eq!(Path::new("/a_file.txt").normalize().is_dir(), false);
    /// ```
    #[inline]
    pub fn is_dir(&self) -> bool {
        self.inner.is_dir()
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

    /// Returns the final component of the `Path`, if there is one.
    ///
    /// For more details see [`Path::file_name`].
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use std::ffi::OsStr;
    /// use ring_utils::Normalize;
    ///
    /// assert_eq!(Some(OsStr::new("foo.txt")), Path::new("/tmp/foo.txt").normalize().file_name());
    /// assert_eq!(None, Path::new("/").normalize().file_name());
    /// ```
    #[inline]
    pub fn file_name(&self) -> Option<&OsStr> {
        self.inner.file_name()
    }

    /// Extracts the extension (without the leading dot) of [`self.file_name`], if possible.
    ///
    /// For more details see [`Path::extension`].
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// assert_eq!(Some("rs".as_ref()), Path::new("/foo.rs").normalize().extension());
    /// assert_eq!(Some("gz".as_ref()), Path::new("/foo.tar.gz").normalize().extension());
    /// ```
    #[inline]
    pub fn extension(&self) -> Option<&OsStr> {
        self.inner.extension()
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

impl<P: ?Sized + AsRef<Path>> PartialEq<P> for NormalizedPath {
    fn eq(&self, other: &P) -> bool {
        self.components().eq(other.as_ref().components())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalized Path Buffer
////////////////////////////////////////////////////////////////////////////////

/// Wrapper of [`PathBuf`] representing path without any '.' or '..' components
///
/// # Examples
///
/// You can use [`push`] to build up a `PathBuf` from
/// components:
///
/// ```
/// use ring_utils::NormalizedPathBuf;
/// use std::path::Path;
///
/// let mut path = NormalizedPathBuf::new();
///
/// path.push(r"/");
/// path.push("tmp");
/// path.push("..");
/// path.push("foo");
/// path.push("bar");
///
/// path.set_extension("txt");
///
/// assert_eq!(path, Path::new("/foo/bar.txt"));
/// ```
///
/// However, [`push`] is best used for dynamic situations. This is a better way
/// to do this when you know all of the components ahead of time:
///
/// ```
/// use ring_utils::NormalizedPathBuf;
/// use std::path::Path;
///
/// let path: NormalizedPathBuf = ["/", "tmp", "..", "foo", "bar.txt"].iter().collect();
///
/// assert_eq!(path, Path::new("/foo/bar.txt"));
/// ```
///
/// We can still do better than this! Since these are all strings, we can use
/// [`Normalize`]:
///
/// ```
/// use ring_utils::Normalize;
/// use std::path::Path;
///
/// let path = Path::new("/tmp/../foo/bar.txt").normalize();
///
/// assert_eq!(path, Path::new("/foo/bar.txt"));
/// ```
#[derive(Clone, Default, Debug, Eq, Ord, PartialOrd)]
pub struct NormalizedPathBuf {
    inner: PathBuf,
}

fn normalized_push(path: &mut PathBuf, component: Component) {
    match component {
        Component::Prefix(..) | Component::RootDir => {
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
    #[must_use]
    #[inline]
    pub fn new() -> NormalizedPathBuf {
        NormalizedPathBuf { inner: PathBuf::new() }
    }

    /// Extends `self` with `path`.
    ///
    /// If `path` is absolute, it replaces the current path.
    ///
    /// See [`Path::push`] for more details.
    ///
    /// # Examples
    ///
    /// Pushing a relative path extends the existing path:
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let mut path = Path::new("/tmp").normalize();
    /// path.push("file.bk");
    ///
    /// assert_eq!(path, Path::new("/tmp/file.bk"));
    /// ```
    ///
    /// Pushing an absolute path replaces the existing path:
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let mut path = Path::new("/tmp").normalize();
    /// path.push("/etc");
    ///
    /// assert_eq!(path, Path::new("/etc"));
    /// ```
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

    /// Removes the last component of the path.
    ///
    /// Returns `false` if nothing is done. Otherwise, returns `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let mut p = Path::new("/spirited/away.rs").normalize();
    ///
    /// p.pop();
    /// assert_eq!(p, Path::new("/spirited"));
    /// p.pop();
    /// assert_eq!(p, Path::new("/"));
    /// ```
    #[inline]
    pub fn pop(&mut self) -> bool {
        self.inner.pop()
    }

    /// Updates [`self.file_name`] to `file_name`.
    ///
    /// For more details see [`PathBuf::set_file_name`]
    ///
    /// [`self.file_name`]: Path::file_name
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let mut buf = Path::new("/").normalize();
    /// assert!(buf.file_name() == None);
    ///
    /// buf.set_file_name("foo.txt");
    /// assert!(buf == Path::new("/foo.txt"));
    /// assert!(buf.file_name().is_some());
    ///
    /// buf.set_file_name("bar.txt");
    /// assert!(buf == Path::new("/bar.txt"));
    ///
    /// buf.set_file_name("baz");
    /// assert!(buf == Path::new("/baz"));
    /// ```
    #[inline]
    pub fn set_file_name<S: AsRef<OsStr>>(&mut self, file_name: S) {
        self.inner.set_file_name(file_name);
    }

    /// Updates [`self.extension`] to `Some(extension)` or to `None` if
    /// `extension` is empty.
    ///
    /// For more details see [`PathBuf::set_extension`].
    ///
    /// [`self.extension`]: Path::extension
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let mut p = Path::new("/feel/the").normalize();
    ///
    /// p.set_extension("force");
    /// assert_eq!(p, Path::new("/feel/the.force"));
    ///
    /// p.set_extension("dark.side");
    /// assert_eq!(p, Path::new("/feel/the.dark.side"));
    ///
    /// p.set_extension("cookie");
    /// assert_eq!(p, Path::new("/feel/the.dark.cookie"));
    ///
    /// p.set_extension("");
    /// assert_eq!(p, Path::new("/feel/the.dark"));
    ///
    /// p.set_extension("");
    /// assert_eq!(p, Path::new("/feel/the"));
    ///
    /// p.set_extension("");
    /// assert_eq!(p, Path::new("/feel/the"));
    /// ```
    #[inline]
    pub fn set_extension<S: AsRef<OsStr>>(&mut self, file_name: S) {
        self.inner.set_extension(file_name);
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

impl<P: ?Sized + AsRef<Path>> PartialEq<P> for NormalizedPathBuf {
    fn eq(&self, other: &P) -> bool {
        self.components().eq(other.as_ref().components())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Normalize trait
////////////////////////////////////////////////////////////////////////////////

pub trait Normalize : AsRef<Path> {
    /// Builds a normalized path for current path object
    fn normalize(&self) -> NormalizedPathBuf;

    /// Computes a normalized path from self using given base.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let base = Path::new("/test").normalize();
    /// assert_eq!(Path::new("./foo").resolve(&base), Path::new("/test/foo"));
    /// assert_eq!(Path::new("../foo").resolve(&base), Path::new("/foo"));
    /// assert_eq!(Path::new("/bar").resolve(&base), Path::new("/bar"));
    /// ```
    #[must_use = "allocated path will be lost if left unused"]
    fn resolve(&self, base: &NormalizedPath) -> NormalizedPathBuf {
        let mut ret = NormalizedPathBuf::from(base);
        ret._push(self.as_ref());
        ret
    }
}

impl Normalize for Path {
    /// Builds a normalized path for current path object
    ///
    /// # Panics
    /// If `self` does not start by either a `RootDir` or a `Prefix` component.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use ring_utils::Normalize;
    ///
    /// let path = Path::new("/foo/baz/../bar").normalize();
    /// assert_eq!(path, Path::new("/foo/bar"));
    /// ```
    #[must_use = "allocated path will be lost if left unused"]
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
    use crate::absolute_path;
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

    #[test]
    fn it_should_iterate_over_path_components() {
        let path = Path::new("/foo/bar").normalize();

        let mut components = path.components();
        assert_eq!(components.next(), Some(NormalizedComponent::RootDir));
        assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("foo"))));
        assert_eq!(components.next(), Some(NormalizedComponent::Normal(OsStr::new("bar"))));
        assert_eq!(components.next(), None);

        let mut components = path.components();
        assert_eq!(components.next_back(), Some(NormalizedComponent::Normal(OsStr::new("bar"))));
        assert_eq!(components.next_back(), Some(NormalizedComponent::Normal(OsStr::new("foo"))));
        assert_eq!(components.next_back(), Some(NormalizedComponent::RootDir));
        assert_eq!(components.next_back(), None);
    }

    #[test]
    #[cfg(windows)]
    fn it_should_os_str_representing_path_prefix() {
        let path = Path::new(r"C:\foo\bar").normalize();
        let prefix = path.components().next().unwrap();
        
        assert_eq!(prefix.as_os_str(), OsStr::new("C:"))
    }
    
    #[test]
    fn it_should_compare_every_normalized_components() {
        let path = Path::new("/foo/bar").normalize();

        assert_eq!(path.components(), path.components());
        assert_eq!(path.components(), Path::new("/foo/bar").components());
    }

    #[test]
    fn it_should_normalize_given_path() {
        assert_eq!(absolute_path!("/foo/baz/../bar").normalize(), absolute_path!("/foo/bar"));
        assert_eq!(absolute_path!("/foo/baz/./bar").normalize(), absolute_path!("/foo/baz/bar"));
    }

    #[test]
    #[should_panic(expected = "normalized path must start with either a RootDir or a Prefix")]
    fn it_cannot_normalize_a_path_starting_with_a_normal_component() {
        let _ = Path::new("foo/bar").normalize();
    }

    #[test]
    #[should_panic(expected = "normalized path must start with either a RootDir or a Prefix")]
    fn it_cannot_normalize_a_path_starting_with_a_cur_dir_component() {
        let _ = Path::new("./foo/bar").normalize();
    }

    #[test]
    #[should_panic(expected = "normalized path must start with either a RootDir or a Prefix")]
    fn it_cannot_normalize_a_path_starting_with_a_parent_dir_component() {
        let _ = Path::new("../foo/bar").normalize();
    }
}
