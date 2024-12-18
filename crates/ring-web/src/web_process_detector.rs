use crate::scripts::ScriptFileDetector;
use crate::NodeProcess;
use itertools::Itertools;
use ring_core::{ProcessUnit, ProcessUnitDetector};
use std::collections::VecDeque;
use std::path::Path;
use std::rc::Rc;
use sysinfo::{Pid, Process};
use tracing::{info, trace};

pub struct WebProcessDetector {
    script_file_detector: Rc<ScriptFileDetector>
}

impl WebProcessDetector {
    pub fn new(script_file_detector: Rc<ScriptFileDetector>) -> WebProcessDetector {
        WebProcessDetector { script_file_detector }
    }
}

impl ProcessUnitDetector for WebProcessDetector {
    fn detect(&self, pid: &Pid, process: &Process) -> anyhow::Result<Option<Rc<dyn ProcessUnit>>> {
        let exe = process.exe()
            .and_then(|p| p.file_stem())
            .and_then(|n| n.to_str());

        if exe == Some("node") {
            let mut args = process.cmd()[1..].iter()
                .map(|arg| arg.to_str().unwrap().to_string())
                .collect::<VecDeque<_>>();

            trace!("parsing node args: {}", args.iter().join(" "));

            while !args.is_empty() {
                let arg = args.pop_front().unwrap();

                if arg.starts_with('-') {
                    if ["-r", "--require", "--import"].contains(&arg.as_str()) {
                        args.pop_front();
                    }
                } else {
                    args.push_front(arg);
                    break;
                }
            }

            if let Some(script_path) = args.pop_front().as_deref().map(Path::new) {
                info!("recognized {pid} as a node process");

                let process = if let Some(running_script) = self.script_file_detector.detect_script(script_path)? {
                    NodeProcess::new(*pid, args.into(), Some(running_script))
                } else {
                    args.push_front(script_path.file_name().unwrap().to_str().unwrap().to_string());
                    NodeProcess::new(*pid, args.into(), None)
                };

                return Ok(Some(Rc::new(process)));
            }
        }

        Ok(None)
    }
}