use ring_core_tasks::Task;
use ring_core_units::Unit;
use std::rc::Rc;

/// A rustup process
#[derive(Clone, Copy, Debug)]
pub struct RustupTask;

impl RustupTask {
    pub fn new() -> RustupTask {
        RustupTask
    }
}

impl Default for RustupTask {
    fn default() -> Self {
        Self::new()
    }
}

impl Task for RustupTask {
    fn kind(&self) -> &str {
        "rustup"
    }

    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        None
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: Some(crossterm::style::Color::Rgb { r: 0xe3, g: 0x3b, b: 0x26 }),
            attributes: crossterm::style::Attribute::Dim.into(),
            ..Default::default()
        }
    }
}