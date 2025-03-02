use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// Cache based on absolute path keys.
#[derive(Clone, Debug)]
pub struct PathCache<V> {
    cache: HashMap<PathBuf, V>,
}

impl<V> PathCache<V> {
    /// Creates a new empty path cache
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::cache::PathCache;
    ///
    /// let mut cache: PathCache<String> = PathCache::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns element matching given key, if any.
    ///
    /// Given path is resolved using [`std::path::absolute`] before searching in the cache.
    /// Returns an error if the path resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::cache::PathCache;
    ///
    /// let mut cache: PathCache<String> = PathCache::new();
    /// assert_eq!(cache.get("/toto").unwrap(), None);
    ///
    /// cache.insert("/toto", "toto".to_string()).unwrap();
    /// assert_eq!(cache.get("/toto").unwrap(), Some(&"toto".to_string()));
    /// ```
    pub fn get<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Option<&V>> {
        Ok(self.cache.get(&std::path::absolute(path)?))
    }

    /// Inserts an element at the given key. Returns the previous value if any.
    ///
    /// Given path is resolved using [`std::path::absolute`] before searching in the cache.
    /// Returns an error if the path resolution fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::cache::PathCache;
    ///
    /// let mut cache: PathCache<String> = PathCache::new();
    /// assert!(cache.insert("/toto", "toto".to_string()).is_ok());
    /// ```
    pub fn insert<P: AsRef<Path>>(&mut self, path: P, value: V) -> io::Result<Option<V>> {
        Ok(self.cache.insert(std::path::absolute(path)?, value))
    }

    /// Returns true if cache contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::cache::PathCache;
    ///
    /// let mut cache: PathCache<String> = PathCache::new();
    /// assert!(cache.is_empty());
    ///
    /// cache.insert("/toto", "toto".to_string()).unwrap();
    /// assert!(!cache.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Returns cache size
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::cache::PathCache;
    ///
    /// let mut cache: PathCache<String> = PathCache::new();
    /// assert_eq!(cache.len(), 0);
    ///
    /// cache.insert("/toto", "toto".to_string()).unwrap();
    /// assert_eq!(cache.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
}

impl<V> Default for PathCache<V> {
    #[inline]
    fn default() -> Self {
        PathCache {
            cache: Default::default(),
        }
    }
}