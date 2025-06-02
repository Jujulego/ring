mod task_cache;

pub use crate::task_cache::TaskCache;
pub use ring_core_content::*;
use ring_core_fs::filesystem::LocalFilesystem;
use ring_core_fs::AugmentedFilesystem;
pub use ring_core_modules::*;
pub use ring_core_tasks::*;
pub use ring_core_units::*;
use std::rc::Rc;

/// Holds and manages all modules references
pub struct Core {
    filesystem: Rc<AugmentedFilesystem<LocalFilesystem>>,
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    pub fn new() -> Rc<Self> {
        let registry = Rc::new(RegistryRef::new());
        let filesystem = Rc::new(AugmentedFilesystem::local_filesystem());

        let modules: Vec<Box<dyn Module>> = vec![
            #[cfg(feature = "javascript")]
            Box::new(ring_module_javascript::JavascriptModule::new(filesystem.clone())),
            #[cfg(feature = "json")]
            Box::new(ring_module_json::JsonModule::new(filesystem.clone())),
            #[cfg(feature = "ring")]
            Box::new(ring_module_ring::RingModule::new(registry.clone())),
            #[cfg(feature = "rust")]
            Box::new(ring_module_rust::RustModule::new(filesystem.clone())),
            #[cfg(feature = "shell")]
            Box::new(ring_module_shell::ShellModule::new(registry.clone(), filesystem.clone())),
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new(filesystem.clone())),
            #[cfg(feature = "typescript")]
            Box::new(ring_module_typescript::TypescriptModule::new(filesystem.clone())),
            #[cfg(feature = "yaml")]
            Box::new(ring_module_yaml::YamlModule::new(filesystem.clone())),
        ];

        let core = Rc::new(Core { filesystem, modules });
        registry.store(core.clone());

        core
    }

    #[inline]
    pub fn filesystem_tools(&self) -> Rc<AugmentedFilesystem<LocalFilesystem>> {
        self.filesystem.clone()
    }
}

impl Registry for Core {
    #[inline]
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