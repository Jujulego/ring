use ring_core_language::Language;
use ring_core_modules::Module;
use std::path::Path;

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
            #[cfg(feature = "module-rust")]
            Box::new(ring_module_rust::RustModule::new()),
            #[cfg(feature = "module-toml")]
            Box::new(ring_module_toml::TomlModule::new()),
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
    /// assert_eq!(core.detect_language("src"), None);
    /// assert_eq!(core.detect_language("src/lib.rs"), Some(rust_language()));
    /// ```
    #[inline]
    pub fn detect_language<P: AsRef<Path>>(&self, path: P) -> Option<Language> {
        self._detect_language(path.as_ref())
    }

    fn _detect_language(&self, path: &Path) -> Option<Language> {
        self.modules.iter()
            .flat_map(|module| module.language_detectors())
            .filter_map(|detector| detector.detect_language(path))
            .next()
    }
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}