use crate::javascript_language;
use ring_core_content::{DetectLanguage, Language};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tracing::{instrument, trace};

#[derive(Clone, Debug, Default)]
pub struct JavascriptFileDetector;

impl JavascriptFileDetector {
    /// Creates a new instance of JavascriptFileDetector
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Checks if given path is a javascript file
    ///
    /// # Examples
    ///
    /// ```
    /// use ring_module_javascript::JavascriptFileDetector;
    ///
    /// let detector = JavascriptFileDetector::new();
    /// assert!(detector.is_javascript_file("assets/test.js"));
    /// ```
    #[inline]
    pub fn is_javascript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_javascript_file(path.as_ref())
    }

    fn _is_javascript_file(&self, path: &Path) -> bool {
        trace!("stat {}", path.display());
        if !path.is_file() {
            return false;
        }

        if matches!(path.extension().and_then(OsStr::to_str), Some("js") | Some("jsx") | Some("cjs") | Some("mjs")) {
            return true;
        }

        trace!("read {}", path.display());
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let shebang = reader.lines()
                .map_while(Result::ok)
                .find(|line| line.starts_with("#!"));

            if shebang.is_some_and(|l| l == "#!/usr/bin/env node") {
                return true;
            }
        }


        false
    }
}

impl DetectLanguage for JavascriptFileDetector {
    #[instrument(name = "javascript-file.detect-language", skip_all)]
    fn detect_language(&self, path: &Path) -> Option<Language> {
        if self._is_javascript_file(path) {
            Some(javascript_language())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_detect_javascript_language() {
        let detector = JavascriptFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("assets/test")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.js")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.jsx")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.cjs")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.mjs")), Some(javascript_language()));
    }

    #[test]
    fn it_should_not_detect_javascript_language() {
        let detector = JavascriptFileDetector::new();

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
