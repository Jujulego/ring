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
    metadata: Option<fs::Metadata>,
}

impl ListFilesItem {
    fn from_path(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path).ok();
        Ok(ListFilesItem { path, metadata })
    }

    fn from_entry(entry: fs::DirEntry) -> io::Result<Self> {
        Ok(ListFilesItem {
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

impl lscolors::Colorable for ListFilesItem {
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
pub struct FilesIterator {
    content: ListFilesContent,
    show_all: bool,
}

impl FilesIterator {
    /// Create a new files iterator, using given path.
    ///
    /// Can fail if the given path is a directory. See [`fs::read_dir`] for possible error cause.
    ///
    /// # Example
    /// ```
    /// use ring_cli_list::FilesIterator;
    ///
    /// let files = FilesIterator::new(".".into()).unwrap();
    /// ```
    pub fn new(path: PathBuf) -> io::Result<FilesIterator> {
        let content = if path.is_dir() {
            ListFilesContent::Contents(fs::read_dir(path)?)
        } else {
            ListFilesContent::Path(path)
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
    /// use ring_cli_list::FilesIterator;
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