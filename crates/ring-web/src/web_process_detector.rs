use crate::{NodeProcess, WebUnitDetector};
use ring_process_unit::{ProcessUnit, ProcessUnitDetector};
use std::rc::Rc;
use sysinfo::{Pid, Process};

pub struct WebProcessDetector {
    web_unit_detector: Rc<WebUnitDetector>
}

impl WebProcessDetector {
    pub fn new(web_unit_detector: Rc<WebUnitDetector>) -> WebProcessDetector {
        WebProcessDetector { web_unit_detector }
    }
}

impl ProcessUnitDetector for WebProcessDetector {
    fn detect(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<dyn ProcessUnit>>> {
        let exe = process.exe()
            .and_then(|p| p.file_stem())
            .and_then(|n| n.to_str());

        if exe == Some("node") {
            if let Some(script_path) = process.cmd().get(1) {
                if let Some(running_script) = self.web_unit_detector.detect_script(script_path)? {
                    let args = process.cmd()[2..].iter()
                        .map(|arg| arg.to_str().unwrap())
                        .map(|arg| arg.to_string())
                        .collect();

                    let process = NodeProcess::new(*pid, args, running_script);

                    return Ok(Some(Rc::new(process)));
                }
            }
        }

        Ok(None)
    }
}