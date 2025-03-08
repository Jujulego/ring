use crate::cache::PathCache;
use crate::units::Identifier;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tracing::{debug, warn};

/// Wraps an identifier, and cache its results
#[derive(Clone, Debug)]
pub struct CachedIdentifier<I: Identifier> {
    identifier: I,
    cache: RefCell<PathCache<Rc<I::Unit>>>,
}

impl<I: Identifier> CachedIdentifier<I> {
    pub fn new(identifier: I) -> CachedIdentifier<I> {
        CachedIdentifier {
            identifier,
            cache: RefCell::new(PathCache::new()),
        }
    }

    #[inline]
    pub fn reset<P: AsRef<Path>>(&self, path: P) {
        if let Err(err) = self.cache.borrow_mut().remove(path.as_ref()) {
            warn!("Failed to reset cached unit at {}: {}", path.as_ref().display(), err);
        }
    }

    #[inline]
    pub fn reset_all(&self) {
        self.cache.borrow_mut().clear();
    }
}

impl<I: Identifier> Identifier for CachedIdentifier<I>
where 
    I::Unit: Clone,
{
    type Unit = I::Unit;
    type Output = Rc<I::Unit>;

    fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<Self::Output>> {
        if let Some(cached) = self.cache.borrow().get(path)? {
            debug!("return cached unit for {}", path.display());
            Ok(Some(cached.clone()))
        } else if let Some(unit) = self.identifier.identify_unit(path)? {
            let unit = Rc::new(unit.borrow().clone());

            debug!("cache unit at {}", path.display());
            self.cache.borrow_mut().insert(path, unit.clone())?;

            Ok(Some(unit))
        } else {
            Ok(None)
        }
    }
}