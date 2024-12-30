use crate::{CargoCrateDetector, CargoProcess};
use ring_core::{ProcessUnit, ProcessUnitDetector};
use std::collections::VecDeque;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::info;

/// Detector for cargo processes
#[derive(Clone, Debug, Default)]
pub struct CargoProcessDetector {
    cargo_crate_detector: Rc<CargoCrateDetector>,
}

impl CargoProcessDetector {
    pub fn new(cargo_crate_detector: Rc<CargoCrateDetector>) -> CargoProcessDetector {
        CargoProcessDetector { cargo_crate_detector }
    }

    pub fn detect_process(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<CargoProcess>>> {
        let exe = process.exe()
            .and_then(|p| p.file_stem())
            .and_then(|n| n.to_str());

        if exe == Some("cargo") {
            info!("recognized {pid} as a cargo process");
            let args = process.cmd()[1..].iter()
                .map(|arg| arg.to_str().unwrap().to_string())
                .collect::<VecDeque<_>>();

            let crt = process.cwd().and_then(|cwd| self.cargo_crate_detector.detect_crate(cwd).transpose())
                .transpose()?;
            
            Ok(Some(Rc::new(CargoProcess::new(*pid, args.into(), crt))))
        } else {
            Ok(None)
        }
    }
}

impl ProcessUnitDetector for CargoProcessDetector {
    fn detect(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<dyn ProcessUnit>>> {
        self.detect_process(pid, process)
            .map(|opt| opt.map(|prc| prc as Rc<dyn ProcessUnit>))
    }
}