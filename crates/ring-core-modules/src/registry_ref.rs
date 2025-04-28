use std::cell::OnceCell;
use std::rc::Rc;
use crate::{Module, Registry};

/// Reference to a registry that can be initiated latter
#[derive(Clone)]
pub struct RegistryRef {
    inner: OnceCell<Rc<dyn Registry>>,
}

impl RegistryRef {
    pub fn new() -> RegistryRef {
        RegistryRef { inner: OnceCell::new() }
    }

    pub fn store(&self, registry: Rc<dyn Registry>) {
        if self.inner.set(registry).is_err() {
            panic!("registry ref is already initialized");
        }
    }
}

impl Default for RegistryRef {
    fn default() -> RegistryRef {
        RegistryRef::new()
    }
}

impl Registry for RegistryRef {
    fn modules(&self) -> &[Box<dyn Module>] {
        self.inner.get()
            .map(|rc| rc.modules())
            .unwrap()
    }
}