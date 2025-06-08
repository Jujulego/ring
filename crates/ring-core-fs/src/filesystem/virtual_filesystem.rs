use crate::traits::{AbsReader, FileMetadata, Filesystem};
use crate::{Error, LocationType};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Virtual filesystem that lives inside memory
#[derive(Debug, Default)]
pub struct VirtualFilesystem {
    content: HashMap<PathBuf, VirtualContent>,
}

impl VirtualFilesystem {
    /// Creates a new instance of virtual filesystem
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a new virtual file
    #[inline]
    pub fn add_file<P: AsRef<Path>, S: Display>(&mut self, path: P, content: S) {
        self._add_file(Path::new("/").join(path), content.to_string());
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

impl FileMetadata for VirtualFilesystem {
    fn location_type(&self, path: &Path) -> Result<LocationType, Error> {
        let path = Path::new("/").join(path);

        match self.content.get(&path) {
            Some(content) => Ok(content.into()),
            None => Err(Error::NotFound("Location not found"))
        }
    }

    #[inline]
    fn is_symlink(&self, _path: &Path) -> bool {
        false
    }
}

impl Filesystem for VirtualFilesystem {
    type File = VirtualFile;

    fn open(&self, path: &Path) -> Result<Self::File, Error> {
        let path = Path::new("/").join(path);

        match self.content.get(&path) {
            Some(VirtualContent::File(content)) => Ok(VirtualFile::new(content.clone())),
            Some(_) | None => Err(Error::NotFound("File not found or is not a file"))
        }
    }
}

/// Virtual filesystem element
#[derive(Clone, Debug)]
enum VirtualContent {
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

/// Virtual file
pub struct VirtualFile {
    content: String,
}

impl VirtualFile {
    #[inline]
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl AbsReader for VirtualFile {
    #[inline]
    fn abs_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self.content.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_file_should_add_file_and_parent_directories() {
        let mut virtual_fs = VirtualFilesystem::new();

        virtual_fs.add_file("/foo/bar/baz", "baz");

        assert!(virtual_fs.is_dir(Path::new("/")));
        assert!(virtual_fs.is_dir(Path::new("/foo")));
        assert!(virtual_fs.is_dir(Path::new("/foo/bar")));
        assert!(virtual_fs.is_file(Path::new("/foo/bar/baz")));
    }

    #[test]
    fn open_should_allow_to_read_given_content() {
        let mut virtual_fs = VirtualFilesystem::new();

        virtual_fs.add_file("/foo/bar/baz", "baz");
        let mut file = virtual_fs.open(Path::new("/foo/bar/baz")).unwrap();

        assert_eq!(std::io::read_to_string(file.abs_reader()).unwrap(), "baz");

        assert!(matches!(virtual_fs.open(Path::new("/foo/toto")), Err(Error::NotFound("File not found or is not a file"))));
        assert!(matches!(virtual_fs.open(Path::new("/foo/bar")), Err(Error::NotFound("File not found or is not a file"))));
    }
}