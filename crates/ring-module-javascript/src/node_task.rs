use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_core_tasks::Task;
use ring_core_units::Unit;
use crate::NpmPackage;

/// A node process
#[derive(Clone, Debug)]
pub struct NodeTask {
    id: String,
    node_exe: PathBuf,
    npm_package: Option<Rc<NpmPackage>>,
    script: Option<PathBuf>,
}

impl NodeTask {
    /// Create a new node task
    pub fn new(id: String, node_exe: PathBuf, script: Option<PathBuf>, npm_package: Option<Rc<NpmPackage>>) -> NodeTask {
        NodeTask { id, script, node_exe, npm_package }
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
        &self.id
    }

    fn executable(&self) -> &Path {
        &self.node_exe
    }

    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    fn kind(&self) -> &str {
        "node"
    }

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