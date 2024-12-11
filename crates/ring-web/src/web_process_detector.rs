use crate::{NodeProcess, WebUnitDetector};
use ring_process_unit::{ProcessUnit, ProcessUnitDetector};
use std::path::Path;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::info;

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
            let mut args = &process.cmd()[1..];

            while !args.is_empty() {
                let arg = args[0].to_str().unwrap();

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

            if let Some(script_path) = args.first().map(Path::new) {
                info!("recognized {pid} as a node process");

                let mut args: Vec<_> = process.cmd()[2..].iter()
                    .map(|arg| arg.to_str().unwrap())
                    .map(|arg| arg.to_string())
                    .collect();

                let process = if let Some(running_script) = self.web_unit_detector.detect_script(script_path)? {
                    NodeProcess::new(*pid, args, Some(running_script))
                } else {
                    args.insert(0, script_path.file_name().unwrap().to_str().unwrap().to_string());
                    NodeProcess::new(*pid, args, None)
                };

                return Ok(Some(Rc::new(process)));
            }
        }

        Ok(None)
    }
}