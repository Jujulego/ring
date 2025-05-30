use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// A ring process
#[derive(Clone)]
pub struct RingTask {
    process: ProcessData,
    working_unit: Option<Rc<dyn Unit>>,
}

impl RingTask {
    /// Creates a new ring task
    #[inline]
    pub fn new(process: ProcessData, working_unit: Option<Rc<dyn Unit>>) -> RingTask {
        RingTask { process, working_unit }
    }
}

impl Task for RingTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"ring"`
    #[inline]
    fn kind(&self) -> &str {
        "ring"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0xff, g: 0xd7, b: 0x00 })
    }
}

impl ProcessTask for RingTask {
    /// Returns the ring executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory ring is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the unit ring is working in, if any
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn ring_task_should_be_a_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/ring"),
            vec!["ring".to_string(), "ls".to_string()],
        );

        let task = RingTask::new(data, None);

        assert_eq!(task.id(), "test");
        assert_eq!(task.kind(), "ring");
        assert_eq!(task.color(), Some(Rgb { r: 0xff, g: 0xd7, b: 0x00 }));
    }

    #[test]
    fn ring_task_should_be_a_process_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/ring"),
            vec!["ring".to_string(), "ls".to_string()],
        );

        let task = RingTask::new(data, None);

        assert_eq!(task.executable(), Path::new("/test/ring"));
        assert_eq!(task.args(), &["ring".to_string(), "ls".to_string()]);
        assert_eq!(task.working_directory(), Path::new("/test"));
        assert!(task.working_unit().is_none());
    }
}