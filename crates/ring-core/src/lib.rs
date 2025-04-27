mod task_cache;

pub use crate::task_cache::TaskCache;
use ring_core_file::FileContent;
use ring_core_modules::Module;
pub use ring_core_modules::Registry;
use ring_core_units::Unit;
use std::path::{absolute, Path};
use std::rc::Rc;

/// Holds and manages all modules references
pub struct Core {
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    /// Initiate all modules
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// 
    /// let core = Core::new();
    /// ```
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

    /// Uses all modules to detect units at given path
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    ///
    /// let core = Core::new();
    /// assert!(!core.detect_units(".").is_empty());
    /// ```
    #[inline]
    pub fn detect_units<P: AsRef<Path>>(&self, path: P) -> Vec<Rc<dyn Unit>> {
        if let Ok(path) = absolute(path.as_ref()) {
            self._detect_units(&path)
        } else {
            vec![]
        }
    }

    fn _detect_units(&self, path: &Path) -> Vec<Rc<dyn Unit>> {
        self.modules.iter()
            .flat_map(|module| module.unit_detectors())
            .filter_map(|detector| detector.detect_unit(path))
            .collect()
    }

    /// Uses all modules to qualify given path's content
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// use ring_core_file::FileContent;
    /// use ring_module_rust::rust_language;
    ///
    /// let core = Core::new();
    /// assert_eq!(core.qualify_file("Cargo.toml"), Some(FileContent::Manifest));
    /// ```
    #[inline]
    pub fn qualify_file<P: AsRef<Path>>(&self, path: P) -> Option<FileContent> {
        self._qualify_file(&absolute(path).ok()?)
    }

    fn _qualify_file(&self, path: &Path) -> Option<FileContent> {
        self.modules.iter()
            .flat_map(|module| module.path_qualifiers())
            .filter_map(|detector| detector.qualify_file(path))
            .max_by_key(|(_, qualified_path)| qualified_path.components().count())
            .map(|(content, _)| content)
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