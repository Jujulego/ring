use rgb::Rgb;
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

    /// Returns a color associated with the task
    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        None
    }

    /// Returns a crossterm style object
    #[cfg(feature = "crossterm")]
    #[inline]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: self.color()
                .map(|color| crossterm::style::Color::Rgb { r: color.r, g: color.g, b: color.b }),
            ..Default::default()
        }
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

        fn color(&self) -> Option<Rgb<u8>> {
            Some(Rgb { r: 0, g: 255, b: 0})
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
        let style = TestTask.style();

        assert_eq!(style.foreground_color, Some(crossterm::style::Color::Rgb { r: 0, g: 255, b: 0 }));
    }
}