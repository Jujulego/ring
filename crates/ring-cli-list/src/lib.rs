use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{fs, io, mem};

enum ListFilesContent {
    Path(PathBuf),
    Contents(fs::ReadDir),
    Empty,
}

pub struct ListFilesItem {
    path: PathBuf,
    metadata: fs::Metadata,
}

impl ListFilesItem {
    fn from_path(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;
        Ok(ListFilesItem { path, metadata })
    }

    fn from_entry(entry: fs::DirEntry) -> io::Result<Self> {
        Ok(ListFilesItem { path: entry.path(), metadata: entry.metadata()? })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(OsStr::to_str)
    }

    pub fn metadata(&self) -> &fs::Metadata {
        &self.metadata
    }
}

impl lscolors::Colorable for ListFilesItem {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn file_name(&self) -> OsString {
        self.path().file_name().unwrap_or_default().to_owned()
    }

    fn file_type(&self) -> Option<fs::FileType> {
        Some(self.metadata().file_type())
    }

    fn metadata(&self) -> Option<fs::Metadata> {
        Some(self.metadata.clone())
    }
}

/// Iterates over files within given path.
/// If the given path is a file, return this file.
pub struct ListFilesIterator {
    content: ListFilesContent,
    show_all: bool,
}

impl ListFilesIterator {
    pub fn new(path: PathBuf) -> io::Result<ListFilesIterator> {
        let content = if path.is_dir() {
            ListFilesContent::Contents(fs::read_dir(path)?)
        } else {
            ListFilesContent::Path(path)
        };

        Ok(ListFilesIterator {
            content,
            show_all: false,
        })
    }

    pub fn enable_show_all(&mut self) {
        self.show_all = true;
    }
}

impl Iterator for ListFilesIterator {
    type Item = io::Result<ListFilesItem>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.content {
            ListFilesContent::Path(path) => {
                let path = mem::take(path);
                self.content = ListFilesContent::Empty;

                Some(ListFilesItem::from_path(path))
            }
            ListFilesContent::Contents(inner) => {
                for entry in inner {
                    match entry {
                        Ok(entry) => {
                            if !self.show_all && entry.file_name().to_str().unwrap_or_default().starts_with(".") {
                                continue;
                            }

                            return Some(ListFilesItem::from_entry(entry));
                        }
                        Err(error) => {
                            return Some(Err(error));
                        }
                    }
                }

                self.content = ListFilesContent::Empty;
                None
            }
            ListFilesContent::Empty => None
        }
    }
}
