mod local_filesystem;
mod virtual_filesystem;

pub use local_filesystem::LocalFilesystem;
pub use virtual_filesystem::{VirtualFile, VirtualFilesystem};
