use crate::NpmPackage;
use ring_core_tasks::{ProcessData, Task};
use ring_core_units::Unit;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// A node process
#[derive(Clone, Debug)]
pub struct NodeTask {
    process: ProcessData,
    npm_package: Option<Rc<NpmPackage>>,
    script: Option<PathBuf>,
}

impl NodeTask {
    /// Create a new node task
    pub fn new(process: ProcessData, script: Option<PathBuf>, npm_package: Option<Rc<NpmPackage>>) -> NodeTask {
        NodeTask { process, script, npm_package }
    }

    /// Returns the package this task is working in
    pub fn npm_package(&self) -> Option<&Rc<NpmPackage>> {
        self.npm_package.as_ref()
    }
}

impl Task for NodeTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns `"node"`
    #[inline]
    fn kind(&self) -> &str {
        "node"
    }

    /// Returns the directory node is working in
    #[inline]
    fn cwd(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the node executable
    #[inline]
    fn exe(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the executed script
    #[inline]
    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    /// Returns the unit node is working in, if any
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.npm_package.as_ref()
            .map(|pt| pt.clone() as Rc<dyn Unit>)
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: Some(crossterm::style::Color::Rgb { r: 0x5f, g: 0xa0, b: 0x4e }),
            ..Default::default()
        }
    }
}