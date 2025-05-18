use ring_core_units::Unit;
use std::rc::Rc;

/// Detected process
pub trait Task {
    /// Task identifier
    fn id(&self) -> &str;

    /// Returns the task's kind
    fn kind(&self) -> &str;

    /// Returns the unit the task is working in
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        None
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
    use super::*;

    struct TestTask;

    impl Task for TestTask {
        fn id(&self) -> &str {
            "id"
        }

        fn kind(&self) -> &str {
            "kind"
        }
    }

    #[test]
    fn working_unit_should_return_none_by_default() {
        let task = TestTask;

        assert!(task.working_unit().is_none());
    }

    #[cfg(feature = "crossterm")]
    #[test]
    fn style_should_return_default_style_by_default() {
        let task = TestTask;

        assert_eq!(task.style(), Default::default());
    }
}