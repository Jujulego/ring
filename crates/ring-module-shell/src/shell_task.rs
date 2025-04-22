use std::path::{Path, PathBuf};
use std::rc::Rc;
use ring_core_tasks::Task;
use ring_core_units::Unit;

/// Shell kind detected
#[derive(Clone, Copy, Debug)]
pub enum ShellKind {
    Bash,
    Cmd,
    Shell,
    PowerShell,
}

/// A shell process
#[derive(Clone, Debug)]
pub struct ShellTask {
    shell_kind: ShellKind,
    exe: PathBuf,
}

impl ShellTask {
    pub fn new(shell_kind: ShellKind, exe: PathBuf) -> ShellTask {
        ShellTask { shell_kind, exe }
    }

    pub fn shell_kind(&self) -> &ShellKind {
        &self.shell_kind
    }
}

impl Task for ShellTask {
    fn exe(&self) -> &Path {
        &self.exe
    }

    fn kind(&self) -> &str {
        match self.shell_kind {
            ShellKind::Bash => "bash",
            ShellKind::Cmd => "cmd",
            ShellKind::Shell => "shell",
            ShellKind::PowerShell => "powershell",
        }
    }

    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        None
    }

    #[cfg(feature = "crossterm")]
    fn style(&self) -> crossterm::style::ContentStyle {
        crossterm::style::ContentStyle {
            foreground_color: match self.shell_kind() {
                ShellKind::PowerShell => Some(crossterm::style::Color::Rgb { r: 0x42, g: 0x72, b: 0xc9 }),
                _ => None,
            },
            ..Default::default()
        }
    }
}