use std::path::{Path, PathBuf};
use sysinfo::Process;

/// Extract of data from a process, used for tasks.
#[derive(Clone, Debug)]
pub struct ProcessData {
    id: String,
    cwd: PathBuf,
    exe: PathBuf,
}

impl ProcessData {
    #[inline]
    pub fn new(id: String, cwd: PathBuf, exe: PathBuf) -> ProcessData {
        ProcessData { id, cwd, exe }
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }

    #[inline]
    pub fn exe(&self) -> &Path {
        &self.exe
    }
}

impl From<&Process> for ProcessData {
    fn from(process: &Process) -> ProcessData {
        ProcessData {
            id: format!("process:{}", process.pid()),
            cwd: process.cwd().unwrap().to_path_buf(),
            exe: process.exe().unwrap().to_path_buf(),
        }
    }
}
