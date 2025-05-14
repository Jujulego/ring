use std::path::{Path, PathBuf};
use ring_core_tasks::Task;
use ring_core_units::Unit;
use std::rc::Rc;

/// A rustup process
#[derive(Clone, Debug)]
pub struct RustupTask {
    id: String,
    exe: PathBuf,
}

impl RustupTask {
    /// Creates a new rustup task
    pub fn new(id: String, exe: PathBuf) -> RustupTask {
        RustupTask { id, exe }
    }
}

impl Task for RustupTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        &self.id
    }

    /// Returns path to the rustup executable
    fn executable(&self) -> &Path {
        &self.exe
    }

    /// Returns `"rustup"`
    fn kind(&self) -> &str {
        "rustup"
    }

    /// Returns none, there is no meaning full unit for rustup task. 
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