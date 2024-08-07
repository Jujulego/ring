use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, trace};
use ring_utils::PathTree;
use crate::CargoManifest;
use crate::constants::MANIFEST;

#[derive(Debug)]
pub struct CargoLoader {
    cache: RefCell<PathTree<Option<Rc<CargoManifest>>>>,
}

impl CargoLoader {
    pub fn new() -> CargoLoader {
        CargoLoader {
            cache: RefCell::new(PathTree::new())
        }
    }
    
    pub fn load(&self, path: &Path) -> anyhow::Result<Option<Rc<CargoManifest>>> {
        if let Some(result) = self.cache.borrow().get(path) {
            if result.is_some() {
                debug!("Loaded cargo manifest at {} (cached)", path.display());
            }
            
            return Ok(result.clone());
        }
        
        let manifest_file = path.join(MANIFEST);

        trace!("Testing {}", manifest_file.display());
        let manifest_exists = manifest_file.try_exists()
            .with_context(|| format!("Unable to access {}", manifest_file.display()))?;

        if manifest_exists {
            let manifest = CargoManifest::parse_file(&manifest_file)?;
            debug!("Loaded cargo manifest at {}", path.display());

            let manifest = Rc::new(manifest);
            self.cache.borrow_mut().set(path, Some(manifest.clone()));
            
            Ok(Some(manifest))
        } else {
            self.cache.borrow_mut().set(path, None);

            Ok(None)
        }
    }
}