use std::cell::RefCell;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::rc::Rc;
use anyhow::{anyhow, Context};
use tracing::{debug, trace};
use ring_traits::{Manifest, OptionalResult};
use ring_utils::PathTree;

#[derive(Debug)]
pub struct ManifestLoader<M : Manifest> {
    filename: &'static str,
    cache: RefCell<PathTree<Option<Rc<M>>>>,
}

impl<M : Manifest> ManifestLoader<M> {
    pub fn new(filename: &'static str) -> ManifestLoader<M> {
        ManifestLoader {
            filename,
            cache: RefCell::new(PathTree::new())
        }
    }

    pub fn load(&self, path: &Path) -> OptionalResult<Rc<M>> {
        let path = path.join(self.filename);
        
        if let Some(result) = self.cache.borrow().get(&path) {
            if result.is_some() {
                debug!("Loaded manifest at {} (cached)", path.display());
            }

            return result.clone().into();
        }
        
        trace!("Try loading {}", path.display());
        match File::open(&path) {
            Ok(ref mut file) => {
                M::from_reader(file)
                    .map(|mnf| Rc::new(mnf))
                    .inspect(|mnf| {
                        debug!("Loaded manifest at {}", path.display());
                        self.cache.borrow_mut().set(&path, Some(mnf.clone()));
                    })
                    .with_context(|| format!("Error while parsing {}", path.display())).into()
            }
            Err(err) if err.kind() == ErrorKind::NotFound => {
                self.cache.borrow_mut().set(&path, None);
                OptionalResult::Empty
            }
            Err(err) => OptionalResult::Fail(
                anyhow!(err).context(format!("Unable to access {}", path.display()))
            )
        }
    }
}
