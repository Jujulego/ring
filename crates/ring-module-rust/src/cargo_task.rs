use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_core_tasks::Task;
use ring_core_units::Unit;
use crate::cargo_crate::CargoCrate;

/// A cargo process
#[derive(Clone, Debug)]
pub struct CargoTask {
    id: String,
    exe: PathBuf,
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl CargoTask {
    /// Create a new cargo task
    pub fn new(id: String, exe: PathBuf, cargo_crate: Option<Rc<CargoCrate>>) -> CargoTask {
        CargoTask { id, exe, cargo_crate }
    }

    /// Returns the crate this task is working in
    pub fn cargo_crate(&self) -> Option<&Rc<CargoCrate>> {
        self.cargo_crate.as_ref()
    }
}

impl Task for CargoTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        &self.id
    }

    /// Returns path to the cargo executable
    fn exe(&self) -> &Path {
        &self.exe
    }

    /// Returns `"cargo"`
    fn kind(&self) -> &str {
        "cargo"
    }

    /// Returns the crate this task is working in
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.cargo_crate.as_ref()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: Some(crossterm::style::Color::Rgb { r: 0xe3, g: 0x3b, b: 0x26 }),
            ..Default::default()
        }
    }
}