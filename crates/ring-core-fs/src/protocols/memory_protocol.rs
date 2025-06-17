use crate::traits::{FilesystemProtocol, Location, LocationIterator, LocationMetadata};
use crate::{FsError, LocationType};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Virtual filesystem that lives inside memory
#[derive(Debug, Default)]
pub struct MemoryProtocol {
    content: HashMap<PathBuf, VirtualContent>,
}

impl MemoryProtocol {
    /// Creates a new instance of virtual filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new virtual file
    #[inline]
    pub fn with_file<P: AsRef<Path>, S: Display>(mut self, path: P, content: S) -> Self {
        self._add_file(Path::new("/").join(path), content.to_string());
        self
    }

    fn _add_file(&mut self, path: PathBuf, content: String) {
        let content = VirtualContent::File(content);

        // Add all ancestors as directories
        path.ancestors()
            .skip(1)
            .for_each(|ancestor| {
                if !self.content.contains_key(ancestor) {
                    self.content.insert(ancestor.to_path_buf(), VirtualContent::Directory);
                }
            });

        // Add the file itself
        self.content.insert(path, content);
    }
}

impl LocationMetadata for MemoryProtocol {
    fn location_type(&self, path: &Path) -> Result<LocationType, FsError> {
        let path = Path::new("/").join(path);

        match self.content.get(&path) {
            Some(content) => Ok(content.into()),
            None => Err(FsError::NotFound("Location not found"))
        }
    }

    #[inline]
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
}

impl FilesystemProtocol for MemoryProtocol {
    #[inline]
    fn locate_path(&self, path: &Path) -> Result<Box<dyn Location>, FsError> {
        let path = Path::new("/").join(path);

        match self.content.get(&path) {
            Some(content) => Ok(Box::new(VirtualLocation::new(path, content.clone())) as _),
            None => Err(FsError::NotFound("File not found or is not a file"))
        }
    }

    fn list_content(&self, path: &Path) -> Result<LocationIterator<'_>, FsError> {
        let path = Path::new("/").join(path);

        let iter = self.content.iter()
            .filter(move |(key, _)| {
                let mut path = path.components();
                let mut key = key.components();

                loop {
                    match (path.next(), key.next()) {
                        (Some(p), Some(k)) if p == k => continue,
                        (None, Some(_)) => break path.next().is_none() && key.next().is_none(),
                        (_, _) => break false,
                    }
                }
            })
            .map(|(key, content)| Box::new(VirtualLocation::new(key.clone(), content.clone())))
            .map(|location| Ok::<Box<dyn Location>, FsError>(location));

        Ok(Box::new(iter) as _)
    }
}

/// Virtual filesystem element
#[derive(Clone, Debug)]
pub enum VirtualContent {
    Directory,
    File(String),
}

impl From<&VirtualContent> for LocationType {
    #[inline]
    fn from(content: &VirtualContent) -> Self {
        match content {
            VirtualContent::Directory => LocationType::Directory,
            VirtualContent::File(_) => LocationType::File,
        }
    }
}

/// Virtual filesystem location
#[derive(Debug)]
pub struct VirtualLocation {
    path: PathBuf,
    content: VirtualContent,
}

impl VirtualLocation {
    #[inline]
    fn new(path: PathBuf, content: VirtualContent) -> Self {
        Self { path, content }
    }
}

impl Location for VirtualLocation {
    #[inline]
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[inline]
    fn read(&mut self) -> Result<Box<dyn Read + '_>, FsError> {
        match &self.content {
            VirtualContent::File(content) => Ok(Box::new(content.as_bytes()) as _),
            VirtualContent::Directory => Err(FsError::NotAFile("This virtual content is not a file")),
        }
    }

    #[cfg(feature = "lscolors")]
    fn indicator(&self) -> lscolors::Indicator {
        match &self.content {
            VirtualContent::Directory => lscolors::Indicator::Directory,
            VirtualContent::File(_) => lscolors::Indicator::RegularFile,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_should_add_file_and_parent_directories() {
        let memory = MemoryProtocol::new()
            .with_file("/foo/bar/baz", "baz");

        assert!(memory.is_dir(Path::new("/")));
        assert!(memory.is_dir(Path::new("/foo")));
        assert!(memory.is_dir(Path::new("/foo/bar")));
        assert!(memory.is_file(Path::new("/foo/bar/baz")));
    }

    #[test]
    fn protocol_should_allow_to_read_given_content() {
        let memory = MemoryProtocol::new()
            .with_file("/foo/bar/baz", "baz");

        // Existing file
        let mut location = memory.locate_path(Path::new("/foo/bar/baz")).unwrap();

        assert_eq!(location.read_to_string().unwrap(), "baz");

        // Existing directory
        let mut location = memory.locate_path(Path::new("/foo/bar")).unwrap();

        assert!(matches!(location.read(), Err(FsError::NotAFile(_))));

        // Not existing path
        assert!(matches!(memory.locate_path(Path::new("/foo/toto")), Err(FsError::NotFound(_))));
    }

    #[test]
    fn protocol_should_allow_to_read_directory() {
        let memory = MemoryProtocol::new()
            .with_file("/foo/toto", "toto")
            .with_file("/foo/bar/baz", "baz");

        let mut locations = memory.list_content(Path::new("/foo")).unwrap()
            .map(|location| location.unwrap().path())
            .collect::<Vec<_>>();

        locations.sort();

        assert_eq!(locations, vec![PathBuf::from("/foo/bar"), PathBuf::from("/foo/toto")]);
    }
}