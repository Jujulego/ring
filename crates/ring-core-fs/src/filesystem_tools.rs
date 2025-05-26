use crate::{adaptators, PathAdaptator};
use std::path::Path;
use crate::file_wrapper::FileWrapper;

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
    fn is_dir(&self, path: &Path) -> bool {
        self.select_adaptator(path).is_dir(path)
    }

    #[inline]
    fn is_file(&self, path: &Path) -> bool {
        self.select_adaptator(path).is_file(path)
    }

    #[inline]
    fn is_supported(&self, path: &Path) -> bool {
        self.adaptators.iter()
            .any(|adaptator| adaptator.is_supported(path))
    }

    #[inline]
    fn open(&self, path: &Path) -> anyhow::Result<Box<dyn FileWrapper + '_>> {
        self.select_adaptator(path).open(path)
    }
}

impl Default for FilesystemTools {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}