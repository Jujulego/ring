use std::fs::FileType;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocationType {
    File,
    Directory,
    Symlink,
}

impl From<FileType> for LocationType {
    fn from(file_type: FileType) -> Self {
        if file_type.is_file() {
            LocationType::File
        } else if file_type.is_dir() {
            LocationType::Directory
        } else if file_type.is_symlink() {
            LocationType::Symlink
        } else {
            panic!("Unsupported file type {file_type:?}")
        }
    }
}