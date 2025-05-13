use crate::shell_task::{ShellKind, ShellTask};
use ring_core_modules::{RegistryRef, UnitRegistry};
use ring_core_tasks::{DetectTask, Task};
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use sysinfo::Process;
use tracing::instrument;

#[derive(Clone)]
pub struct ShellTaskDetector {
    registry: Rc<RegistryRef>,
}

impl ShellTaskDetector {
    pub fn new(registry: Rc<RegistryRef>) -> ShellTaskDetector {
        ShellTaskDetector {
            registry
        }
    }

    pub fn detect_shell_kind(&self, exe: &Path) -> Option<ShellKind> {
        match exe.file_stem().and_then(OsStr::to_str) {
            Some("bash") => Some(ShellKind::Bash),
            Some("cmd") => Some(ShellKind::Cmd),
            Some("sh") => Some(ShellKind::Shell),
            Some("pwsh") | Some("powershell") => Some(ShellKind::PowerShell),
            Some("zsh") => Some(ShellKind::Zsh),
            _ => None
        }
    }
}

impl DetectTask for ShellTaskDetector {
    #[instrument(name = "shell-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        let exe = process.exe()?;
        let shell_kind = self.detect_shell_kind(exe)?;
        
        let task = Rc::new(ShellTask::new(
            format!("process:{}", process.pid()),
            shell_kind,
            exe.to_path_buf(),
            process.cwd()
                .and_then(|cwd| self.registry.detect_units_containing(cwd).first().cloned())
        ));

        Some(task)
    }
}