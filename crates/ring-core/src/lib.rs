use ring_core_file::{FileContent, Language};
use ring_core_modules::Module;
use ring_core_units::Unit;
use std::path::{absolute, Path};
use std::rc::Rc;
use sysinfo::Process;
use ring_core_tasks::Task;

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
            #[cfg(feature = "toml")]
            Box::new(ring_module_toml::TomlModule::new()),
            #[cfg(feature = "typescript")]
            Box::new(ring_module_typescript::TypescriptModule::new()),
            #[cfg(feature = "yaml")]
            Box::new(ring_module_yaml::YamlModule::new()),
        ];

        Core { modules }
    }

    /// Uses all modules to detect given path's language
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_core::Core;
    /// use ring_module_rust::rust_language;
    ///
    /// let core = Core::new();
    /// assert_eq!(core.detect_language("src/lib.rs"), Some(rust_language()));
    /// ```
    #[inline]
    pub fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        self._detect_language(&absolute(path).ok()?)
    }

    fn _detect_language(&self, path: &Path) -> Option<Language> {
        self.modules.iter()
            .flat_map(|module| module.language_detectors())
            .filter_map(|detector| detector.detect_language(path))
            .next()
    }

    /// Uses all modules to detect task of given process
    pub fn detect_tasks(&self, process: &Process) -> Option<Rc<dyn Task>> {
        self.modules.iter()
            .flat_map(|module| module.task_detectors())
            .filter_map(|detector| detector.detect_task(process))
            .next()
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

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}