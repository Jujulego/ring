use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{fs, io, mem};
use tracing::trace;

#[derive(Debug)]
enum FilesContent {
    Path(PathBuf),
    Contents(Box<fs::ReadDir>),
    Empty,
}

#[derive(Clone, Debug)]
pub struct FilesItem {
    path: PathBuf,
    metadata: Option<fs::Metadata>,
}

impl FilesItem {
    fn from_path(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path).ok();
        Ok(FilesItem { path, metadata })
    }

    fn from_entry(entry: fs::DirEntry) -> io::Result<Self> {
        Ok(FilesItem {
            path: entry.path(),
            metadata: entry.metadata().ok()
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn file_name(&self) -> Option<&str> {
        self.path.file_name().and_then(OsStr::to_str)
    }

    pub fn metadata(&self) -> Option<&fs::Metadata> {
        self.metadata.as_ref()
    }
}

impl lscolors::Colorable for FilesItem {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn file_name(&self) -> OsString {
        self.path().file_name().unwrap_or_default().to_owned()
    }

    fn file_type(&self) -> Option<fs::FileType> {
        self.metadata().map(fs::Metadata::file_type)
    }

    fn metadata(&self) -> Option<fs::Metadata> {
        self.metadata.clone()
    }
}

/// Iterates over files within given path.
/// If the given path is a file, return this file.
#[derive(Debug)]
pub struct FilesIterator {
    content: FilesContent,
    show_all: bool,
}

impl FilesIterator {
    /// Create a new files iterator, using given path.
    ///
    /// Can fail if the given path is a directory. See [`fs::read_dir`] for possible error cause.
    ///
    /// # Example
    /// ```
    /// use ring_cli_fs::FilesIterator;
    ///
    /// let files = FilesIterator::new(".".into()).unwrap();
    /// ```
    pub fn new(path: PathBuf) -> io::Result<FilesIterator> {
        let content = if path.is_dir() {
            trace!("reading directory {}", path.display());
            FilesContent::Contents(Box::new(fs::read_dir(path)?))
        } else {
            FilesContent::Path(path)
        };

        Ok(FilesIterator {
            content,
            show_all: false,
        })
    }

    /// Enables show all mode. Hidden file will no longer be filtered.
    /// A "hidden" file is a file with a name starting by a "."
    ///
    /// # Example
    /// ```
    /// use ring_cli_fs::FilesIterator;
    ///
    /// let mut files = FilesIterator::new(".".into()).unwrap();
    /// files.enable_show_all();
    /// ```
    #[inline]
    pub fn enable_show_all(&mut self) {
        self.show_all = true;
    }
}

impl Iterator for FilesIterator {
    type Item = io::Result<FilesItem>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.content {
            FilesContent::Path(path) => {
                let path = mem::take(path);
                self.content = FilesContent::Empty;

                Some(FilesItem::from_path(path))
            }
            FilesContent::Contents(inner) => {
                for entry in inner {
                    match entry {
                        Ok(entry) => {
                            if !self.show_all && entry.file_name().to_str().unwrap_or_default().starts_with(".") {
                                continue;
                            }

                            return Some(FilesItem::from_entry(entry));
                        }
                        Err(error) => {
                            return Some(Err(error));
                        }
                    }
                }

                self.content = FilesContent::Empty;
                None
            }
            FilesContent::Empty => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.content {
            FilesContent::Path(_) => (1, Some(1)),
            FilesContent::Contents(inner) => inner.size_hint(),
            FilesContent::Empty => (0, Some(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_list_not_hidden_files() {
        let files = FilesIterator::new(".".into()).unwrap();
        
        let mut files = files
            .filter_map(Result::ok)
            .filter_map(|item| item.file_name().map(String::from))
            .collect::<Vec<_>>();

        files.sort();
        
        assert_eq!(files, vec!["Cargo.toml".to_string(), "src".to_string()]);
    }

    #[test]
    fn it_should_list_all_files() {
        let mut files = FilesIterator::new(".".into()).unwrap();
        files.enable_show_all();

        let mut files = files
            .filter_map(Result::ok)
            .filter_map(|item| item.file_name().map(String::from))
            .collect::<Vec<_>>();
        
        files.sort();

        assert_eq!(files, vec![".hidden".to_string(), "Cargo.toml".to_string(), "src".to_string()]);
    }

    #[test]
    fn it_should_only_list_given_file() {
        let mut files = FilesIterator::new(".hidden".into()).unwrap();
        files.enable_show_all();

        let mut files = files
            .filter_map(Result::ok)
            .filter_map(|item| item.file_name().map(String::from))
            .collect::<Vec<_>>();

        files.sort();

        assert_eq!(files, vec![".hidden".to_string()]);
    }
}