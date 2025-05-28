use crate::{Error, FileWrapper, PathAdaptator};
use std::collections::HashMap;
use std::fmt::Display;
use std::io::Read;
use std::path::{Path, PathBuf};

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

/// Virtual filesystem that lives inside memory
#[derive(Clone, Debug)]
pub struct VirtualFilesystem {
    content: HashMap<PathBuf, VirtualContent>,
}

impl VirtualFilesystem {
    #[inline]
    pub fn new() -> Self {
        VirtualFilesystem {
            content: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_file<P: AsRef<Path>, S: Display>(&mut self, path: P, content: S) {
        self._add_file(path.as_ref().to_path_buf(), content.to_string());
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

impl PathAdaptator for VirtualFilesystem {
    #[inline]
    fn is_dir(&self, path: &Path) -> bool {
        self.content.get(path).is_some_and(|c| c.is_dir())
    }

    #[inline]
    fn is_file(&self, path: &Path) -> bool {
        self.content.get(path).is_some_and(|c| c.is_file())
    }

    #[inline]
    fn is_supported(&self, _path: &Path) -> bool {
        true
    }

    fn open(&self, path: &Path) -> Result<Box<dyn FileWrapper + '_>, Error> {
        match self.content.get(path) {
            Some(VirtualContent::File(content)) => Ok(Box::new(VirtualFile { content })),
            Some(_) | None => Err(Error::NotFound("File not found or is not a file"))
        }
    }
}

pub struct VirtualFile<'a> {
    content: &'a String,
}

impl<'a> FileWrapper for VirtualFile<'a> {
    fn reader(&mut self) -> Result<Box<dyn Read + '_>, Error> {
        Ok(Box::new(self.content.as_bytes()))
    }
}

impl Default for VirtualFilesystem {
    fn default() -> Self {
        Self::new()
    }
}