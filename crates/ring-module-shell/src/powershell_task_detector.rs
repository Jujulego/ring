use crate::powershell_task::PowershellTask;
use ring_core_modules::{RegistryRef, UnitRegistry};
use ring_core_tasks::{DetectProcessTask, ProcessData, ProcessTask};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use sysinfo::Process;
use tracing::instrument;

#[derive(Clone)]
pub struct PowershellTaskDetector {
    registry: Rc<RegistryRef>,
}

impl PowershellTaskDetector {
    #[inline]
    pub fn new(registry: Rc<RegistryRef>) -> PowershellTaskDetector {
        PowershellTaskDetector {
            registry
        }
    }

    pub fn is_powershell(&self, exe: &Path) -> bool {
        matches!(exe.file_stem().and_then(OsStr::to_str), Some("pwsh") | Some("powershell"))
    }
}

impl DetectProcessTask for PowershellTaskDetector {
    #[instrument(name = "powershell-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn ProcessTask>> {
        let exe = process.exe()?;

        if self.is_powershell(exe) {
            let data = ProcessData::from(process);
            let script = extract_script(&data.cmd()[1..]);
            let unit = self.registry.detect_units_containing(data.cwd()).first().cloned();

            let task = Rc::new(PowershellTask::new(data, script, unit));

            Some(task)
        } else {
            None
        }
    }
}

fn extract_script(cmd: &[String]) -> Option<PathBuf> {
    let (idx, _) = cmd.iter()
        .enumerate()
        .find(|&(_, s)| s == "-File")?;

    cmd.get(idx + 1).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_none() {
        assert_eq!(extract_script(&[]), None);
    }

    #[test]
    fn it_should_return_script_path() {
        assert_eq!(
            extract_script(&["toto".to_string(), "-File".to_string(), "script.ps1".to_string()]),
            Some(PathBuf::from("script.ps1"))
        );
    }
}