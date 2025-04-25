use ring_core_tasks::Task;
use ring_core_units::Unit;
use std::path::{Path, PathBuf};
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
#[derive(Clone, Debug)]
pub struct ShellTask {
    shell_kind: ShellKind,
    exe: PathBuf,
}

impl ShellTask {
    /// Creates a new shell task
    #[inline]
    pub fn new(shell_kind: ShellKind, exe: PathBuf) -> ShellTask {
        ShellTask { shell_kind, exe }
    }

    /// Returns the kind of shell running in the task
    #[inline]
    pub fn shell_kind(&self) -> &ShellKind {
        &self.shell_kind
    }
}

impl Task for ShellTask {
    /// Returns the shell executable
    #[inline]
    fn exe(&self) -> &Path {
        &self.exe
    }

    /// Returns the kind of shell running in the task
    #[inline]
    fn kind(&self) -> &str {
        self.shell_kind.into()
    }

    /// Returns none
    #[inline]
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