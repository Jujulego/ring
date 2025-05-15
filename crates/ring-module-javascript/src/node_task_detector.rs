use crate::node_task::NodeTask;
use crate::NpmPackageDetector;
use ring_core_tasks::{DetectTask, Task};
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::rc::Rc;
use sysinfo::Process;
use tracing::{debug, instrument, warn};

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
            let script = extract_node_script(&process.cmd()[1..])
                .map(PathBuf::from);
            
            let script = script.zip(process.cwd())
                .map(|(script, cwd)| cwd.join(script));

            let package = process.cwd()
                .and_then(|cwd| self.npm_package_detector.load_package_containing(cwd).transpose())
                .transpose()?;

            let task = NodeTask::new(process.into(), script, package);

            return Ok(Some(Rc::new(task)))
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

fn extract_node_script(mut args: &[OsString]) -> Option<&OsStr> {
    while !args.is_empty() {
        let arg = args.first().unwrap().to_str().unwrap();

        if arg.starts_with('-') {
            if ["-r", "--require", "--import"].contains(&arg) {
                args = &args[2..];
            } else {
                args = &args[1..];
            }
        } else {
            break;
        }
    }

    args.first()
        .map(|str| str.as_os_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_extract_node_script() {
        let args = ["-r", "toto", "test.js"].map(OsString::from);

        assert_eq!(extract_node_script(&args[2..]).and_then(|str| str.to_str()), Some("test.js"));
        assert_eq!(extract_node_script(&args).and_then(|str| str.to_str()), Some("test.js"));
    }
}