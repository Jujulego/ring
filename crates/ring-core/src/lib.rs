mod task_cache;

pub use crate::task_cache::TaskCache;
pub use ring_core_file::*;
pub use ring_core_modules::*;
pub use ring_core_tasks::*;
pub use ring_core_units::*;

/// Holds and manages all modules references
pub struct Core {
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    pub fn new() -> Self {
        let modules: Vec<Box<dyn Module>> = vec![
            #[cfg(feature = "javascript")]
            Box::new(ring_module_javascript::JavascriptModule::new()),
            #[cfg(feature = "json")]
            Box::new(ring_module_json::JsonModule::new()),
            #[cfg(feature = "rust")]
            Box::new(ring_module_rust::RustModule::new()),
            #[cfg(feature = "shell")]
            Box::new(ring_module_shell::ShellModule::new()),
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new()),
            #[cfg(feature = "typescript")]
            Box::new(ring_module_typescript::TypescriptModule::new()),
            #[cfg(feature = "yaml")]
            Box::new(ring_module_yaml::YamlModule::new()),
        ];

        Core { modules }
    }
}

impl Registry for Core {
    fn modules(&self) -> &[Box<dyn Module>] {
        &self.modules
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
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