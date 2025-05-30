use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
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

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0xe3, g: 0x3b, b: 0x26 })
    }
}

impl ProcessTask for RustupTask {
    /// Returns path to the rustup executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory rustup is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }
}