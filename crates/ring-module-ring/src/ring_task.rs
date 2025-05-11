use ring_core_units::Unit;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_core_tasks::Task;

/// A ring process
#[derive(Clone)]
pub struct RingTask {
    exe: PathBuf,
    working_unit: Option<Rc<dyn Unit>>,
}

impl RingTask {
    /// Creates a new ring task
    #[inline]
    pub fn new(exe: PathBuf, working_unit: Option<Rc<dyn Unit>>) -> RingTask {
        RingTask { exe, working_unit }
    }
}

impl Task for RingTask {
    /// Returns the ring executable
    #[inline]
    fn exe(&self) -> &Path {
        &self.exe
    }

    /// Returns "ring"
    #[inline]
    fn kind(&self) -> &str {
        "ring"
    }

    /// Returns detected unit
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.clone()
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: Some(crossterm::style::Color::Rgb { r: 0xff, g: 0xd7, b: 0x00 }),
            ..Default::default()
        }
    }
}