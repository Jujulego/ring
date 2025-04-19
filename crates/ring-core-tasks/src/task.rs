use std::rc::Rc;
use ring_core_units::Unit;

/// Detected process
pub trait Task {
    /// Returns the task's kind
    fn kind(&self) -> &str;

    /// Returns the unit the task is working in
    fn working_unit(&self) -> Option<Rc<dyn Unit>>;

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        Default::default()
    }
}