use ring_core_tasks::{ProcessData, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// A ring process
#[derive(Clone)]
pub struct RingTask {
    process: ProcessData,
    working_unit: Option<Rc<dyn Unit>>,
}

impl RingTask {
    /// Creates a new ring task
    #[inline]
    pub fn new(process: ProcessData, working_unit: Option<Rc<dyn Unit>>) -> RingTask {
        RingTask { process, working_unit }
    }
}

impl Task for RingTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"ring"`
    #[inline]
    fn kind(&self) -> &str {
        "ring"
    }

    /// Returns the directory ring is working in
    #[inline]
    fn cwd(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the ring executable
    #[inline]
    fn exe(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the unit ring is working in, if any
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