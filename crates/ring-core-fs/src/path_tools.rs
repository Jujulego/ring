use std::path::Path;
use crate::{adaptators, PathAdaptator};

pub struct PathTools {
    adaptators: Vec<Box<dyn PathAdaptator>>,
}

impl PathTools {
    pub fn new() -> Self {
        PathTools {
            adaptators: vec![
                Box::new(adaptators::YarnArchives),
                Box::new(adaptators::Filesystem)
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

impl PathAdaptator for PathTools {
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

impl Default for PathTools {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}