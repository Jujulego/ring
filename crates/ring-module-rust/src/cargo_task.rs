use crate::cargo_crate::CargoCrate;
use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// A cargo process
#[derive(Clone, Debug)]
pub struct CargoTask {
    process: ProcessData,
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl CargoTask {
    /// Create a new cargo task
    pub fn new(process: ProcessData, cargo_crate: Option<Rc<CargoCrate>>) -> CargoTask {
        CargoTask { process, cargo_crate }
    }

    /// Returns the crate this task is working in
    pub fn cargo_crate(&self) -> Option<&Rc<CargoCrate>> {
        self.cargo_crate.as_ref()
    }
}

impl Task for CargoTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"cargo"`
    #[inline]
    fn kind(&self) -> &str {
        "cargo"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 })
    }
}

impl ProcessTask for CargoTask {
    /// Returns path to the cargo executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory cargo is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the crate this task is working in
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.cargo_crate()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn cargo_task_should_be_a_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/exe"),
            vec!["exe".to_string(), "-a".to_string()],
        );

        let task = CargoTask::new(data, None);

        assert_eq!(task.id(), "test");
        assert_eq!(task.kind(), "cargo");
        assert_eq!(task.color(), Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 }));
    }

    #[test]
    fn cargo_task_should_be_a_process_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/exe"),
            vec!["exe".to_string(), "-a".to_string()],
        );

        let task = CargoTask::new(data, None);

        assert_eq!(task.executable(), Path::new("/test/exe"));
        assert_eq!(task.args(), &["exe".to_string(), "-a".to_string()]);
        assert_eq!(task.working_directory(), Path::new("/test"));
        assert!(task.working_unit().is_none());
    }
}