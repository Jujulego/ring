use crate::{adaptators, PathAdaptator};
use std::path::Path;

pub struct FilesystemTools {
    adaptators: Vec<Box<dyn PathAdaptator>>,
}

impl FilesystemTools {
    pub fn new() -> Self {
        FilesystemTools {
            adaptators: vec![
                Box::new(adaptators::ArchivesAdaptator::new()),
                Box::new(adaptators::FilesystemAdaptator)
            ]
        }
    }

    fn select_adaptator(&self, path: &Path) -> &dyn PathAdaptator {
        self.adaptators.iter()
            .find(|adaptator| adaptator.is_supported(path))
            .map(|adaptator| adaptator.as_ref())
            .unwrap()
    }
}

impl PathAdaptator for FilesystemTools {
    #[inline]
    fn is_supported(&self, path: &Path) -> bool {
        self.adaptators.iter()
            .any(|adaptator| adaptator.is_supported(path))
    }

    #[inline]
    fn is_file(&self, path: &Path) -> anyhow::Result<bool> {
        self.select_adaptator(path).is_file(path)
    }
}

impl Default for FilesystemTools {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}