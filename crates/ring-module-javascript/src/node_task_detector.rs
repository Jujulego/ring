use std::ffi::OsStr;
use std::path::PathBuf;
use std::rc::Rc;
use sysinfo::Process;
use tracing::{debug, instrument, warn};
use ring_core_tasks::{DetectTask, Task};
use crate::node_task::NodeTask;
use crate::NpmPackageDetector;

#[derive(Clone, Debug)]
pub struct NodeTaskDetector {
    npm_package_detector: Rc<NpmPackageDetector>,
}

impl NodeTaskDetector {
    /// Creates a new `NodeTaskDetector` instance
    pub fn new(npm_package_detector: Rc<NpmPackageDetector>) -> NodeTaskDetector {
        NodeTaskDetector {
            npm_package_detector,
        }
    }

    /// Checks if given process is a node task
    pub fn is_node_task(&self, process: &Process) -> bool {
        process.exe()
            .is_some_and(|exe| exe.file_stem().and_then(OsStr::to_str) == Some("node"))
    }

    /// Builds a `NodeTask` object from given process.
    pub fn load_node_task(&self, process: &Process) -> anyhow::Result<Option<Rc<NodeTask>>> {
        if self.is_node_task(process) {
            if let Some(cwd) = process.cwd() {
                let script = process.cmd().get(1)
                    .map(PathBuf::from)
                    .unwrap_or_else(|| process.exe().unwrap().to_path_buf());

                let script = cwd.join(script);
                let package = self.npm_package_detector.load_package_containing(&script)?;

                let task = NodeTask::new(script, package);

                return Ok(Some(Rc::new(task)))
            }
        }

        Ok(None)
    }
}

impl DetectTask for NodeTaskDetector {
    #[instrument(name = "node-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        match self.load_node_task(process) {
            Ok(opt) => opt.map(|t| t as Rc<dyn Task>),
            Err(err) => {
                warn!("{}", err);
                if let Some(source) = err.source() {
                    debug!("Error caused by: {}", source);
                }

                None
            }
        }
    }
}