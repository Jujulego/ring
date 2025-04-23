use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;
use sysinfo::Process;
use tracing::instrument;
use ring_core_tasks::{DetectTask, Task};
use crate::shell_task::{ShellKind, ShellTask};

#[derive(Clone, Copy, Debug)]
pub struct ShellTaskDetector;

impl ShellTaskDetector {
    pub fn new() -> ShellTaskDetector {
        ShellTaskDetector
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

impl Default for ShellTaskDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl DetectTask for ShellTaskDetector {
    #[instrument(name = "shell-task.detect-task", skip_all)]
    fn detect_task(&self, process: &Process) -> Option<Rc<dyn Task>> {
        let exe = process.exe()?;
        let shell_kind = self.detect_shell_kind(exe)?;
        let task = Rc::new(ShellTask::new(shell_kind, exe.to_path_buf()));

        Some(task)
    }
}