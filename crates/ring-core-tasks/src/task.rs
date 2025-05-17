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