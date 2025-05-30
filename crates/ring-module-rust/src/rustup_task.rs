use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use std::path::Path;

/// A rustup process
#[derive(Clone, Debug)]
pub struct RustupTask {
    process: ProcessData
}

impl RustupTask {
    /// Creates a new rustup task
    pub fn new(process: ProcessData) -> RustupTask {
        RustupTask {
            process
        }
    }
}

impl Task for RustupTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"rustup"`
    #[inline]
    fn kind(&self) -> &str {
        "rustup"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 })
    }
}

impl ProcessTask for RustupTask {
    /// Returns path to the rustup executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory rustup is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    
    #[test]
    fn rustup_task_should_be_a_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/rustup"),
            vec!["rustup".to_string(), "-a".to_string()],
        );
        
        let task = RustupTask::new(data);
        
        assert_eq!(task.id(), "test");
        assert_eq!(task.kind(), "rustup");
        assert_eq!(task.color(), Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 }));
    }
    
    #[test]
    fn rustup_task_should_be_a_process_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/rustup"),
            vec!["rustup".to_string(), "-a".to_string()],
        );

        let task = RustupTask::new(data);

        assert_eq!(task.executable(), Path::new("/test/rustup"));
        assert_eq!(task.args(), &["rustup".to_string(), "-a".to_string()]);
        assert_eq!(task.working_directory(), Path::new("/test"));
    }
}