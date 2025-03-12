use ring_core_language::Language;
use ring_core_modules::Module;
use std::path::Path;

/// Holds and manages all modules references
pub struct Core {
    modules: Vec<Box<dyn Module>>,
}

impl Core {
    pub fn new() -> Self {
        let mut modules: Vec<Box<dyn Module>> = Vec::new();

        #[cfg(feature = "rust")]
        modules.push(Box::new(ring_module_rust::RustModule::new()));

        Core { modules }
    }
}

#[inline]
pub fn detect_language<P: AsRef<Path>>(core: &Core, path: P) -> Option<Language> {
    _detect_language(core, path.as_ref())
}

fn _detect_language(core: &Core, path: &Path) -> Option<Language> {
    core.modules.iter()
        .flat_map(|module| module.language_detectors())
        .filter_map(|detector| detector.detect_language(path))
        .next()
}
