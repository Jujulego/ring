mod task_cache;

pub use crate::task_cache::TaskCache;
pub use ring_core_content::*;
use ring_core_fs::FilesystemTools;
pub use ring_core_modules::*;
pub use ring_core_tasks::*;
pub use ring_core_units::*;
use std::rc::Rc;

/// Holds and manages all modules references
pub struct Core {
    filesystem_tools: Rc<FilesystemTools>,
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    pub fn new() -> Rc<Self> {
        let registry = Rc::new(RegistryRef::new());
        let filesystem_tools = Rc::new(FilesystemTools::new());

        let modules: Vec<Box<dyn Module>> = vec![
            #[cfg(feature = "javascript")]
            Box::new(ring_module_javascript::JavascriptModule::new()),
            #[cfg(feature = "json")]
            Box::new(ring_module_json::JsonModule::new(filesystem_tools.clone())),
            #[cfg(feature = "ring")]
            Box::new(ring_module_ring::RingModule::new(registry.clone())),
            #[cfg(feature = "rust")]
            Box::new(ring_module_rust::RustModule::new()),
            #[cfg(feature = "shell")]
            Box::new(ring_module_shell::ShellModule::new(registry.clone(), filesystem_tools.clone())),
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new(filesystem_tools.clone())),
            #[cfg(feature = "typescript")]
            Box::new(ring_module_typescript::TypescriptModule::new()),
            #[cfg(feature = "yaml")]
            Box::new(ring_module_yaml::YamlModule::new(filesystem_tools.clone())),
        ];

        let core = Rc::new(Core { filesystem_tools, modules });
        registry.store(core.clone());

        core
    }
    
    pub fn path_tools(&self) -> Rc<FilesystemTools> {
        self.filesystem_tools.clone()
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