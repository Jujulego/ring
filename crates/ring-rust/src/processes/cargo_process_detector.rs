use crate::CargoProcess;
use ring_core::{ProcessUnit, ProcessUnitDetector};
use std::collections::VecDeque;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::info;

/// Detector for cargo processes
#[derive(Clone, Debug, Default)]
pub struct CargoProcessDetector {}

impl CargoProcessDetector {
    pub fn new() -> CargoProcessDetector {
        CargoProcessDetector {}
    }

    pub fn detect_process(&self, pid: &Pid, process: &Process) -> Option<Rc<CargoProcess>> {
        let exe = process.exe()
            .and_then(|p| p.file_stem())
            .and_then(|n| n.to_str());

        if exe == Some("cargo") {
            info!("recognized {pid} as a cargo process");
            let args = process.cmd()[1..].iter()
                .map(|arg| arg.to_str().unwrap().to_string())
                .collect::<VecDeque<_>>();
            
            Some(Rc::new(CargoProcess::new(*pid, args.into())))
        } else {
            None
        }
    }
}

impl ProcessUnitDetector for CargoProcessDetector {
    fn detect(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<dyn ProcessUnit>>> {
        Ok(self.detect_process(pid, process).map(|prc| prc as Rc<dyn ProcessUnit>))
    }
}