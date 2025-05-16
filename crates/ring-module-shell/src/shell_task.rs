use ring_core_tasks::{ProcessData, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// Shell kind detected
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellKind {
    Bash,
    Cmd,
    Shell,
    PowerShell,
    Zsh,
}

impl From<ShellKind> for &'static str {
    fn from(kind: ShellKind) -> Self {
        match kind {
            ShellKind::Bash => "bash",
            ShellKind::Cmd => "cmd",
            ShellKind::Shell => "shell",
            ShellKind::PowerShell => "powershell",
            ShellKind::Zsh => "zsh",
        }
    }
}

/// A shell process
#[derive(Clone)]
pub struct ShellTask {
    process: ProcessData,
    shell_kind: ShellKind,
    working_unit: Option<Rc<dyn Unit>>,
}

impl ShellTask {
    /// Creates a new shell task
    #[inline]
    pub fn new(process: ProcessData, shell_kind: ShellKind, working_unit: Option<Rc<dyn Unit>>) -> ShellTask {
        ShellTask { process, shell_kind, working_unit }
    }

    /// Returns the kind of shell running in the task
    #[inline]
    pub fn shell_kind(&self) -> &ShellKind {
        &self.shell_kind
    }
}

impl Task for ShellTask {
    /// Returns the process pid
    #[inline]
    fn id(&self) -> &str {
        self.process.id()
    }

    /// Returns the kind of shell running in the task
    #[inline]
    fn kind(&self) -> &str {
        self.shell_kind.into()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory shell is working in
    #[inline]
    fn cwd(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns the shell executable
    #[inline]
    fn exe(&self) -> &Path {
        self.process.exe()
    }

    /// Returns detected unit
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.clone()
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

#[cfg(test)]
mod tests {
    use crate::shell_task::ShellKind;

    #[test]
    fn it_should_return_shell_kind_as_string() {
        assert_eq!({ let t: &str = ShellKind::Bash.into(); t }, "bash");
        assert_eq!({ let t: &str = ShellKind::Cmd.into(); t }, "cmd");
        assert_eq!({ let t: &str = ShellKind::Shell.into(); t }, "shell");
        assert_eq!({ let t: &str = ShellKind::PowerShell.into(); t }, "powershell");
        assert_eq!({ let t: &str = ShellKind::Zsh.into(); t }, "zsh");
    }
}