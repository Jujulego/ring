use crate::process_task_data::ProcessTaskData;
use crate::Task;
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// Detected process
pub trait ProcessTask: Task {
    /// Path to the running executable
    fn executable(&self) -> &Path;

    /// Path to the directory the task is working in
    fn working_directory(&self) -> &Path;

    /// Returns the command line use to invoke the task
    #[inline]
    fn args(&self) -> &[String] {
        &[]
    }

    /// Path to the script ran by the executable, if any
    #[inline]
    fn script(&self) -> Option<&Path> {
        None
    }

    /// Returns the unit containing the script
    #[inline]
    fn script_unit(&self) -> Option<Rc<dyn Unit>> {
        None
    }

    /// Builds a [`ProcessTaskData`] object from the current task
    #[inline]
    fn inspect(&self) -> ProcessTaskData {
        ProcessTaskData {
            id: self.id().to_string(),
            kind: self.kind().to_string(),
            args: self.args().to_vec(),
            working_directory: self.working_directory().to_path_buf(),
            working_unit: self.working_unit().map(|u| u.inspect()),
            executable: self.executable().to_path_buf(),
            script: self.script().map(|p| p.to_path_buf()),
            script_unit: self.script_unit().map(|u| u.inspect()),
            color: self.color(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    struct TestTask;

    impl Task for TestTask {
        fn id(&self) -> &str {
            "id"
        }

        fn kind(&self) -> &str {
            "kind"
        }
    }

    impl ProcessTask for TestTask {
        fn executable(&self) -> &Path {
            "/test/exe".as_ref()
        }

        fn working_directory(&self) -> &Path {
            "/test".as_ref()
        }
    }

    #[test]
    fn args_should_return_an_empty_slice_by_default() {
        let task = TestTask;

        assert!(task.args().is_empty());
    }

    #[test]
    fn script_should_return_none_by_default() {
        let task = TestTask;

        assert!(task.script().is_none());
    }

    #[test]
    fn script_unit_should_return_none_by_default() {
        let task = TestTask;

        assert!(task.script_unit().is_none());
    }

    #[test]
    fn working_unit_should_return_none_by_default() {
        let task = TestTask;

        assert!(task.working_unit().is_none());
    }

    #[test]
    fn inspect_should_build_data_using_other_methods() {
        let task = TestTask;
        let data = task.inspect();

        assert_eq!(data.id, "id");
        assert_eq!(data.kind, "kind");
        assert_eq!(data.args, Vec::<String>::new());
        assert_eq!(data.working_directory, PathBuf::from("/test"));
        assert_eq!(data.executable, PathBuf::from("/test/exe"));
        assert!(data.script.is_none());
        assert!(data.script_unit.is_none());
        assert!(data.working_unit.is_none());
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn style_should_return_default_style_by_default() {
        let task = TestTask;

        assert_eq!(task.style(), Default::default());
    }
}