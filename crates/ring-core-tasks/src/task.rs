use crate::task_data::TaskData;
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// Detected process
pub trait Task {
    /// Task identifier
    fn id(&self) -> &str;

    /// Path to the running executable
    fn exe(&self) -> &Path;

    /// Returns the task's kind
    fn kind(&self) -> &str;

    /// Returns the unit the task is working in
    fn working_unit(&self) -> Option<Rc<dyn Unit>>;

    /// Builds a [`TaskData`] object from the current task
    #[inline]
    fn inspect(&self) -> TaskData {
        TaskData {
            id: self.id().to_string(),
            kind: self.kind().to_string(),
        }
    }
    
    /// Returns a crossterm style object
    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        Default::default()
    }
}