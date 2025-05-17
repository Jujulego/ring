use ring_core_tasks::{ProcessData, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// A rustup process
#[derive(Clone, Debug)]
pub struct RustupTask {
    process: ProcessData
}

impl RustupTask {
    /// Creates a new rustup task
    pub fn new(process: ProcessData) -> RustupTask {
        RustupTask {
            process
        }
    }
}

impl Task for RustupTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"rustup"`
    #[inline]
    fn kind(&self) -> &str {
        "rustup"
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory rustup is working in
    #[inline]
    fn cwd(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns path to the rustup executable
    #[inline]
    fn exe(&self) -> &Path {
        self.process.exe()
    }

    /// Returns none, there is no meaning full unit for rustup task.
    #[inline]
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