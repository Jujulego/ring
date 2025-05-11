mod task_cache;

use std::rc::Rc;
pub use crate::task_cache::TaskCache;
pub use ring_core_content::*;
pub use ring_core_modules::*;
pub use ring_core_tasks::*;
pub use ring_core_units::*;

/// Holds and manages all modules references
pub struct Core {
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    pub fn new() -> Rc<Self> {
        let registry = Rc::new(RegistryRef::new());

        let modules: Vec<Box<dyn Module>> = vec![
            #[cfg(feature = "javascript")]
            Box::new(ring_module_javascript::JavascriptModule::new()),
            #[cfg(feature = "json")]
            Box::new(ring_module_json::JsonModule::new()),
            #[cfg(feature = "ring")]
            Box::new(ring_module_ring::RingModule::new(registry.clone())),
            #[cfg(feature = "rust")]
            Box::new(ring_module_rust::RustModule::new()),
            #[cfg(feature = "shell")]
            Box::new(ring_module_shell::ShellModule::new(registry.clone())),
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new()),
            #[cfg(feature = "typescript")]
            Box::new(ring_module_typescript::TypescriptModule::new()),
            #[cfg(feature = "yaml")]
            Box::new(ring_module_yaml::YamlModule::new()),
        ];

        let core = Rc::new(Core { modules });
        registry.store(core.clone());

        core
    }
}

impl Registry for Core {
    fn modules(&self) -> &[Box<dyn Module>] {
        &self.modules
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_new_core() {
        let core = Core::new();

        assert!(!core.modules().is_empty());
    }
}