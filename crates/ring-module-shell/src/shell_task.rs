use ring_core_tasks::{ProcessData, ProcessTask, Task};
use ring_core_units::Unit;
use std::path::Path;
use std::rc::Rc;

/// Shell kind detected
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellKind {
    Bash,
    Cmd,
    Shell,
    Zsh,
}

impl From<ShellKind> for &'static str {
    fn from(kind: ShellKind) -> Self {
        match kind {
            ShellKind::Bash => "bash",
            ShellKind::Cmd => "cmd",
            ShellKind::Shell => "shell",
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
}

impl ProcessTask for ShellTask {
    /// Returns the shell executable
    #[inline]
    fn executable(&self) -> &Path {
        self.process.exe()
    }

    /// Returns the command line used
    #[inline]
    fn args(&self) -> &[String] {
        self.process.cmd()
    }

    /// Returns the directory shell is working in
    #[inline]
    fn working_directory(&self) -> &Path {
        self.process.cwd()
    }

    /// Returns detected unit
    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_should_return_shell_kind_as_string() {
        assert_eq!({ let t: &str = ShellKind::Bash.into(); t }, "bash");
        assert_eq!({ let t: &str = ShellKind::Cmd.into(); t }, "cmd");
        assert_eq!({ let t: &str = ShellKind::Shell.into(); t }, "shell");
        assert_eq!({ let t: &str = ShellKind::Zsh.into(); t }, "zsh");
    }

    #[test]
    fn shell_task_should_be_a_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/sh"),
            vec!["sh".to_string(), "-a".to_string()],
        );

        let task = ShellTask::new(data, ShellKind::Shell, None);

        assert_eq!(task.id(), "test");
        assert_eq!(task.kind(), "shell");
        assert_eq!(task.color(), None);
    }

    #[test]
    fn shell_task_should_be_a_process_task() {
        let data = ProcessData::new(
            "test".to_string(),
            PathBuf::from("/test"),
            PathBuf::from("/test/sh"),
            vec!["sh".to_string(), "-a".to_string()],
        );

        let task = ShellTask::new(data, ShellKind::Shell, None);

        assert_eq!(task.executable(), Path::new("/test/sh"));
        assert_eq!(task.args(), &["sh".to_string(), "-a".to_string()]);
        assert_eq!(task.working_directory(), Path::new("/test"));
        assert!(task.working_unit().is_none());
    }
}