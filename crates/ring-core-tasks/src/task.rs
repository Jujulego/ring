use crate::task_data::TaskData;
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// Detected process
pub trait Task {
    /// Task identifier
    fn id(&self) -> &str;

    /// Returns the task's kind
    fn kind(&self) -> &str;

    /// Returns the command line use to invoke the task
    #[inline]
    fn args(&self) -> &[String] {
        &[]
    }

    /// Path to the directory the task is working in
    fn cwd(&self) -> &Path;

    /// Path to the running executable
    fn exe(&self) -> &Path;

    /// Path to the script ran by the executable
    #[inline]
    fn script(&self) -> Option<&Path> {
        None
    }

    /// Returns the unit the task is working in
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        None
    }

    /// Builds a [`TaskData`] object from the current task
    #[inline]
    fn inspect(&self) -> TaskData {
        TaskData {
            id: self.id().to_string(),
            kind: self.kind().to_string(),
            args: self.args().to_vec(),
            cwd: self.cwd().to_path_buf(),
            exe: self.exe().to_path_buf(),
            script: self.script().map(|p| p.to_path_buf()),
            working_unit: self.working_unit().map(|u| u.inspect())
        }
    }
    
    /// Returns a crossterm style object
    #[cfg(feature = "crossterm")]
    #[inline]
    fn style(&self) -> crossterm::style::ContentStyle {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    struct TestTask;

    impl Task for TestTask {
        fn id(&self) -> &str {
            "id"
        }

        fn kind(&self) -> &str {
            "kind"
        }

        fn cwd(&self) -> &Path {
            "/test".as_ref()
        }

        fn exe(&self) -> &Path {
            "/test/exe".as_ref()
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
    fn working_unit_should_return_none_by_default() {
        let task = TestTask;

        assert!(task.working_unit().is_none());
    }

    #[test]
    fn inspect_should_return_build_data_using_other_methods() {
        let task = TestTask;
        let data = task.inspect();

        assert_eq!(data.id, "id");
        assert_eq!(data.kind, "kind");
        assert_eq!(data.args, Vec::<String>::new());
        assert_eq!(data.cwd, PathBuf::from("/test"));
        assert_eq!(data.exe, PathBuf::from("/test/exe"));
        assert!(data.script.is_none());
        assert!(data.working_unit.is_none());
    }
    
    #[test]
    fn style_should_return_default_style_by_default() {
        let task = TestTask;

        assert_eq!(task.style(), Default::default());
    }
}