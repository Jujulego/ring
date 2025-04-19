use std::ffi::OsStr;
use std::rc::Rc;
use sysinfo::Process;
use tracing::{debug, instrument, warn};
use ring_core_tasks::{DetectTask, Task};
use crate::cargo_task::CargoTask;
use crate::CargoCrateDetector;

#[derive(Clone, Debug)]
pub struct CargoTaskDetector {
    crate_detector: Rc<CargoCrateDetector>,
}

impl CargoTaskDetector {
    pub fn new(crate_detector: Rc<CargoCrateDetector>) -> Self {
        CargoTaskDetector { crate_detector }
    }

    pub fn is_cargo_task(&self, process: &Process) -> bool {
        process.exe()
            .is_some_and(|exe| exe.file_stem().and_then(OsStr::to_str) == Some("cargo"))
    }

    pub fn load_cargo_task(&self, process: &Process) -> anyhow::Result<Option<Rc<CargoTask>>> {
        if self.is_cargo_task(process) {
            if let Some(cwd) = process.cwd() {
                let unit = self.crate_detector.load_crate_at(cwd)?;

                return Ok(Some(Rc::new(CargoTask::new(unit))))
            }
        }

        Ok(None)
    }
}

impl DetectTask for CargoTaskDetector {
    #[instrument(name = "cargo-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        match self.load_cargo_task(process) {
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