use ring_core_units::Unit;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_core_tasks::Task;

/// A ring process
#[derive(Clone)]
pub struct RingTask {
    id: String,
    exe: PathBuf,
    is_self: bool,
    working_unit: Option<Rc<dyn Unit>>,
}

impl RingTask {
    /// Creates a new ring task
    #[inline]
    pub fn new(id: String, exe: PathBuf, is_self: bool, working_unit: Option<Rc<dyn Unit>>) -> RingTask {
        RingTask { id, exe, is_self, working_unit }
    }
    
    /// Returns true if it is the current process
    pub fn is_self(&self) -> bool {
        self.is_self
    }
}

impl Task for RingTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        &self.id
    }

    /// Returns the ring executable
    #[inline]
    fn executable(&self) -> &Path {
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
            attributes: if self.is_self {
                crossterm::style::Attribute::Underlined.into()
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }
}