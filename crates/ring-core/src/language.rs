use std::path::Path;
use ring_core_language::{DetectLanguage, Language};

#[inline]
pub fn detect_language<P: AsRef<Path>>(path: P) -> Option<Language> {
    _detect_language(path.as_ref())
}

fn _detect_language(path: &Path) -> Option<Language> {
    #[cfg(feature = "rust")] {
        let detector = ring_module_rust::RustFileDetector::new();
        
        if let Some(language) = detector.detect_language(path) {
            return Some(language);
        }
    }
    
    None
}