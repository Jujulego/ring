use std::path::{Path, PathBuf};
use sysinfo::Process;

/// Extract of data from a process, used for tasks.
#[derive(Clone, Debug)]
pub struct ProcessData {
    id: String,
    cmd: Vec<String>,
    cwd: PathBuf,
    exe: PathBuf,
}

impl ProcessData {
    #[inline]
    pub fn new(id: String, cwd: PathBuf, exe: PathBuf, cmd: Vec<String>) -> ProcessData {
        ProcessData { id, cmd, cwd, exe }
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn cmd(&self) -> &[String] {
        &self.cmd
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
        ProcessData::new(
            format!("process:{}", process.pid()),
            process.cwd().unwrap().to_path_buf(),
            process.exe().unwrap().to_path_buf(),
            process.cmd().iter()
                .filter_map(|s| s.to_str())
                .map(|s| s.to_string())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use sysinfo::{Pid, System};
    use super::*;

    #[test]
    fn it_should_extract_data_from_process() {
        let sys = System::new_all();
        let process = sys.process(Pid::from_u32(std::process::id())).unwrap();
        let data = ProcessData::from(process);

        assert_eq!(data.id(), format!("process:{}", process.pid()));
        assert_eq!(data.cmd().len(), process.cmd().len());
        assert_eq!(data.cwd(), process.cwd().unwrap());
        assert_eq!(data.exe(), process.exe().unwrap());
    }
}