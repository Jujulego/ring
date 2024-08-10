use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use anyhow::Context;
use tracing::{debug, trace};
use ring_traits::Manifest;
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
        
        let manifest_path = path.join(MANIFEST);

        trace!("Testing {}", manifest_path.display());
        let manifest_exists = manifest_path.try_exists()
            .with_context(|| format!("Unable to access {}", manifest_path.display()))?;

        if manifest_exists {
            trace!("Parsing manifest file {}", path.display());
            let mut manifest_file = File::open(path)
                .with_context(|| format!("Unable to access {}", manifest_path.display()))?;

            let manifest = CargoManifest::from_reader(&mut manifest_file)
                .with_context(|| format!("Error while parsing {}", manifest_path.display()))?;

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