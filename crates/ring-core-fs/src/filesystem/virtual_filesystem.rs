use crate::traits::{AbstractReader, Filesystem};
use crate::Error;
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

impl Filesystem for VirtualFilesystem {
    type File = VirtualFile;

    fn is_dir(&self, path: &Path) -> bool {
        let path = Path::new("/").join(path);
        self.content.get(&path).is_some_and(|c| c.is_dir())
    }

    fn is_file(&self, path: &Path) -> bool {
        let path = Path::new("/").join(path);
        self.content.get(&path).is_some_and(|c| c.is_file())
    }

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

impl VirtualContent {
    #[inline]
    fn is_dir(&self) -> bool {
        matches!(self, VirtualContent::Directory)
    }

    #[inline]
    fn is_file(&self) -> bool {
        matches!(self, VirtualContent::File(_))
    }
}

/// Virtual file
pub struct VirtualFile {
    content: String,
}

impl VirtualFile {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl AbstractReader for VirtualFile {
    fn as_reader(&mut self) -> Box<dyn Read + '_> {
        Box::new(self.content.as_bytes())
    }
}