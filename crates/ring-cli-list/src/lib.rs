use std::path::PathBuf;
use std::{fs, io, mem};

enum ListFilesContent {
    Path(PathBuf),
    Contents(fs::ReadDir),
    Empty,
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
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.content {
            ListFilesContent::Path(path) => {
                let path = mem::take(path);
                self.content = ListFilesContent::Empty;

                Some(Ok(path))
            }
            ListFilesContent::Contents(inner) => {
                for entry in inner {
                    match entry {
                        Ok(entry) => {
                            if self.show_all || !entry.file_name().to_str().unwrap_or_default().starts_with(".") {
                                return Some(Ok(entry.path()));
                            }
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
