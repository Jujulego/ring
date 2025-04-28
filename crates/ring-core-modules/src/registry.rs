use crate::Module;

/// Object managing a set of modules.
pub trait Registry {
    /// Returns list of registered modules
    fn modules(&self) -> &[Box<dyn Module>];
}
