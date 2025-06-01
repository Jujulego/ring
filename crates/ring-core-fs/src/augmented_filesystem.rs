use crate::filesystem::LocalFilesystem;
use crate::middlewares::ZipMiddleware;
use crate::traits::{AbstractFilesystem, AbstractFilesystemMiddleware};
use crate::Error;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

pub struct AugmentedFilesystem {
    filesystem: Rc<dyn AbstractFilesystem>,
    middlewares: Vec<Box<dyn AbstractFilesystemMiddleware>>,
}

impl AugmentedFilesystem {
    pub fn local_filesystem() -> Self {
        let filesystem = Rc::new(LocalFilesystem::new());

        Self {
            filesystem: filesystem.clone(),
            middlewares: vec![Box::new(ZipMiddleware::new(filesystem))]
        }
    }
}

impl AbstractFilesystem for AugmentedFilesystem {
    fn is_dir(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.is_dir(path))
            .unwrap_or_else(|| self.filesystem.is_dir(path))
    }

    fn is_file(&self, path: &Path) -> bool {
        self.middlewares.iter()
            .find_map(|m| m.is_file(path))
            .unwrap_or_else(|| self.filesystem.is_file(path))
    }

    fn open(&self, path: &Path) -> Result<Box<dyn Read + '_>, Error> {
        self.middlewares.iter()
            .find_map(|m| m.open(path))
            .unwrap_or_else(|| self.filesystem.open(path))
    }
}