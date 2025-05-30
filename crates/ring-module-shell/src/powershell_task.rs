use rgb::Rgb;
use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// A powershell process
#[derive(Clone)]
pub struct PowershellTask {
    process: ProcessData,
    script: Option<PathBuf>,
    working_unit: Option<Rc<dyn Unit>>,
}

impl PowershellTask {
    /// Creates a new powershell task
    #[inline]
    pub fn new(process: ProcessData, script: Option<PathBuf>, working_unit: Option<Rc<dyn Unit>>) -> PowershellTask {
        PowershellTask { process, script, working_unit }
    }
}

impl Task for PowershellTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns [`"powershell"`]
    #[inline]
    fn kind(&self) -> &str {
        "powershell"
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        Some(Rgb { r: 0x42, g: 0x72, b: 0xc9 })
    }
}

impl ProcessTask for PowershellTask {
    /// Returns the shell executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    #[inline]
    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    /// Returns the directory shell is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns detected unit
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.clone()
    }
}
