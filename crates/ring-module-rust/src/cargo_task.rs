use crate::cargo_crate::CargoCrate;
use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// A cargo process
#[derive(Clone, Debug)]
pub struct CargoTask {
    process: ProcessData,
    cargo_crate: Option<Rc<CargoCrate>>,
}

impl CargoTask {
    /// Create a new cargo task
    pub fn new(process: ProcessData, cargo_crate: Option<Rc<CargoCrate>>) -> CargoTask {
        CargoTask { process, cargo_crate }
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
        self.process.id()
    }

    /// Returns `"cargo"`
    #[inline]
    fn kind(&self) -> &str {
        "cargo"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 })
    }
}

impl ProcessTask for CargoTask {
    /// Returns path to the cargo executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory cargo is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the crate this task is working in
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.cargo_crate.as_ref()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }
}