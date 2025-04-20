use std::path::Path;
use std::rc::Rc;
use ring_core_units::Unit;

/// Detected process
pub trait Task {
    /// Path to the running executable
    fn exe(&self) -> &Path;
    
    /// Returns the task's kind
    fn kind(&self) -> &str;

    /// Returns the unit the task is working in
    fn working_unit(&self) -> Option<Rc<dyn Unit>>;
    
    /// Returns a crossterm style object
    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        Default::default()
    }
}