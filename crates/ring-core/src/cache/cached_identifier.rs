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
            return Ok(Some(cached.clone()));
        }

        if let Some(unit) = self.identifier.identify_unit(path)? {
            let unit = Rc::new(unit.borrow().clone());

            debug!("cache unit at {}", path.display());
            self.cache.borrow_mut().insert(path, unit.clone())?;

            Ok(Some(unit))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::{Language, Unit};
    use mockall::mock;

    #[derive(Clone, Debug)]
    struct TestUnit {}

    impl Unit for TestUnit {
        fn language(&self) -> &Language {
            unimplemented!();
        }

        fn path(&self) -> &Path {
            unimplemented!();
        }
    }

    mock! {
        TestIdentifier {}

        impl Identifier for TestIdentifier {
            type Unit = TestUnit;
            type Output = TestUnit;

            fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<TestUnit>>;
        }
    }

    #[test]
    fn it_should_call_successful_wrapped_identifier_once() {
        let mut identifier = MockTestIdentifier::new();
        identifier.expect_identify_unit()
            .times(1)
            .returning(|_| Ok(Some(TestUnit {})));

        let mut cached = CachedIdentifier::new(identifier);

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());

        cached.identifier.checkpoint();
    }

    #[test]
    fn it_should_call_empty_wrapped_identifier_three_times() {
        let mut identifier = MockTestIdentifier::new();
        identifier.expect_identify_unit()
            .times(3)
            .returning(|_| Ok(None));

        let mut cached = CachedIdentifier::new(identifier);

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_none());
        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_none());
        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_none());

        cached.identifier.checkpoint();
    }

    #[test]
    fn it_should_reset_given_path_cache_entry() {
        let mut identifier = MockTestIdentifier::new();
        identifier.expect_identify_unit()
            .times(3)
            .returning(|_| Ok(Some(TestUnit {})));

        let mut cached = CachedIdentifier::new(identifier);

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/tata.rs")).unwrap().is_some());
        cached.reset("/test/toto.rs");

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/tata.rs")).unwrap().is_some());

        cached.identifier.checkpoint();
    }

    #[test]
    fn it_should_ignore_reset_error() {
        let identifier = MockTestIdentifier::new();

        let cached = CachedIdentifier::new(identifier);
        cached.reset("");
    }

    #[test]
    fn it_should_reset_all_cache_entries() {
        let mut identifier = MockTestIdentifier::new();
        identifier.expect_identify_unit()
            .times(4)
            .returning(|_| Ok(Some(TestUnit {})));

        let mut cached = CachedIdentifier::new(identifier);

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/tata.rs")).unwrap().is_some());
        cached.reset_all();

        assert!(cached.identify_unit(Path::new("/test/toto.rs")).unwrap().is_some());
        assert!(cached.identify_unit(Path::new("/test/tata.rs")).unwrap().is_some());

        cached.identifier.checkpoint();
    }
}