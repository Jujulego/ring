use crate::NpmPackage;
use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// A node process
#[derive(Clone, Debug)]
pub struct NodeTask {
    process: ProcessData,
    script: Option<PathBuf>,
    script_package: Option<Rc<NpmPackage>>,
    working_package: Option<Rc<NpmPackage>>,
}

impl NodeTask {
    /// Create a new node task
    pub fn new(
        process: ProcessData,
        script: Option<PathBuf>,
        script_package: Option<Rc<NpmPackage>>,
        working_package: Option<Rc<NpmPackage>>
    ) -> NodeTask {
        NodeTask { process, working_package, script, script_package }
    }

    /// Returns the package this task is working in
    pub fn working_package(&self) -> Option<&Rc<NpmPackage>> {
        self.working_package.as_ref()
    }

    /// Returns the package containing this task's script
    pub fn script_package(&self) -> Option<&Rc<NpmPackage>> {
        self.script_package.as_ref()
    }
}

impl Task for NodeTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"node"`
    #[inline]
    fn kind(&self) -> &str {
        "node"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0x5f, g: 0xa0, b: 0x4e })
    }
}

impl ProcessTask for NodeTask {
    /// Returns the node executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the executed script
    #[inline]
    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    /// Returns the unit containing this task's script
    #[inline]
    fn script_unit(&self) -> Option<Rc<dyn Unit>> {
        self.script_package()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }

    /// Returns the directory node is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the unit node is working in, if any
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_package()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn node_task_should_be_a_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/node"),
            vec!["node".to_string(), "/test/script.js".to_string()],
        );

        let task = NodeTask::new(data, Some(PathBuf::from("/test/script.js")), None, None);

        assert_eq!(task.id(), "test");
        assert_eq!(task.kind(), "node");
        assert_eq!(task.color(), Some(Rgb { r: 0x5f, g: 0xa0, b: 0x4e }));
    }

    #[test]
    fn node_task_should_be_a_process_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/node"),
            vec!["node".to_string(), "/test/script.js".to_string()],
        );

        let task = NodeTask::new(data, Some(PathBuf::from("/test/script.js")), None, None);

        assert_eq!(task.executable(), Path::new("/test/node"));
        assert_eq!(task.args(), &["node".to_string(), "/test/script.js".to_string()]);
        assert_eq!(task.script(), Some(Path::new("/test/script.js")));
        assert!(task.script_unit().is_none());
        assert_eq!(task.working_directory(), Path::new("/test"));
        assert!(task.working_unit().is_none());
    }
}