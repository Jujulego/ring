use crate::{ProcessTask, Task};
use rgb::Rgb;
use ring_core_units::{Unit, UnitData};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Descriptive data of a given task
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProcessTaskData {
    pub id: String,
    pub kind: String,
    pub args: Vec<String>,
    pub color: Option<Rgb<u8>>,
    pub executable: PathBuf,
    pub script: Option<PathBuf>,
    pub working_directory: PathBuf,
    pub working_unit: Option<UnitData>,
}

impl Task for ProcessTaskData {
    #[inline]
    fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    fn kind(&self) -> &str {
        &self.kind
    }

    #[inline]
    fn working_unit(&self) -> Option<Rc<dyn Unit>> {
        self.working_unit.as_ref()
            .map(|unit| Rc::new(unit.clone()) as Rc<dyn Unit>)
    }

    #[inline]
    fn color(&self) -> Option<Rgb<u8>> {
        self.color
    }
}

impl ProcessTask for ProcessTaskData {
    #[inline]
    fn executable(&self) -> &Path {
        &self.executable
    }

    #[inline]
    fn working_directory(&self) -> &Path {
        &self.working_directory
    }

    #[inline]
    fn args(&self) -> &[String] {
        &self.args
    }

    #[inline]
    fn script(&self) -> Option<&Path> {
        self.script.as_deref()
    }

    #[inline]
    fn inspect(&self) -> ProcessTaskData {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_implement_task_trait() {
        let data = ProcessTaskData {
            id: "id".to_string(),
            kind: "kind".to_string(),
            args: vec![],
            color: Some(Rgb { r: 0, g: 0, b: 0 }),
            executable: PathBuf::from("/test/exe"),
            script: Some(PathBuf::from("/test/script")),
            working_directory: PathBuf::from("/test"),
            working_unit: None,
        };

        assert_eq!(data.id(), "id");
        assert_eq!(data.kind(), "kind");
        assert!(data.working_unit().is_none());
        assert_eq!(data.color(), Some(Rgb { r: 0, g: 0, b: 0 }));
    }

    #[test]
    fn it_should_implement_process_task_trait() {
        let data = ProcessTaskData {
            id: "id".to_string(),
            kind: "kind".to_string(),
            args: vec![],
            color: Some(Rgb { r: 0, g: 0, b: 0 }),
            executable: PathBuf::from("/test/exe"),
            script: Some(PathBuf::from("/test/script")),
            working_directory: PathBuf::from("/test"),
            working_unit: None,
        };

        assert_eq!(data.executable(), Path::new("/test/exe"));
        assert_eq!(data.working_directory(), Path::new("/test"));
        assert!(data.args().is_empty());
        assert_eq!(data.script(), Some(Path::new("/test/script")));
    }

    #[test]
    fn inspect_should_return_a_clone() {
        let data = ProcessTaskData {
            id: "id".to_string(),
            kind: "kind".to_string(),
            args: vec![],
            color: Some(Rgb { r: 0, g: 0, b: 0 }),
            executable: PathBuf::from("/test/exe"),
            script: Some(PathBuf::from("/test/script")),
            working_directory: PathBuf::from("/test"),
            working_unit: None,
        };

        assert_eq!(data.id, data.inspect().id);
    }
}