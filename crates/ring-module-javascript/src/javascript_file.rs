use crate::javascript_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct JavascriptFileDetector {
    path_adaptator: Rc<dyn PathAdaptator>,
}

impl JavascriptFileDetector {
    /// Creates a new instance of JavascriptFileDetector
    #[inline]
    pub fn new(path_adaptator: Rc<dyn PathAdaptator>) -> Self {
        Self {
            path_adaptator,
        }
    }

    /// Checks if given path is a javascript file
    #[inline]
    pub fn is_javascript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_javascript_file(path.as_ref())
    }

    fn _is_javascript_file(&self, path: &Path) -> bool {
        if !self.path_adaptator.is_file(path) {
            return false;
        }

        if matches!(path.extension().and_then(OsStr::to_str), Some("js") | Some("jsx") | Some("cjs") | Some("mjs")) {
            return true;
        }

        // Shebangs
        let Ok(mut file) = self.path_adaptator.open(path) else { return false };
        let Ok(reader) = file.reader() else { return false };
        
        let reader = BufReader::new(reader);
        let shebang = reader.lines()
            .map_while(Result::ok)
            .next();

        shebang.is_some_and(|l| l == "#!/usr/bin/env node")
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
    use ring_core_fs::VirtualFilesystem;

    #[test]
    fn it_should_detect_javascript_language_using_extensions() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test.js", "");
        virtual_fs.add_file("test.jsx", "");
        virtual_fs.add_file("test.cjs", "");
        virtual_fs.add_file("test.mjs", "");

        let detector = JavascriptFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_javascript_file(Path::new("test.js")));
        assert!(detector.is_javascript_file(Path::new("test.jsx")));
        assert!(detector.is_javascript_file(Path::new("test.cjs")));
        assert!(detector.is_javascript_file(Path::new("test.mjs")));

        assert_eq!(detector.detect_language(Path::new("test.js")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("test.jsx")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("test.cjs")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("test.mjs")), Some(javascript_language()));
    }

    #[test]
    fn it_should_detect_javascript_language_using_shebang() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("test", "#!/usr/bin/env node");

        let detector = JavascriptFileDetector::new(Rc::new(virtual_fs));

        assert!(detector.is_javascript_file(Path::new("test")));

        assert_eq!(detector.detect_language(Path::new("test")), Some(javascript_language()));
    }

    #[test]
    fn it_should_not_detect_javascript_language() {
        let mut virtual_fs = VirtualFilesystem::new();
        virtual_fs.add_file("src/lib.rs", "");

        let detector = JavascriptFileDetector::new(Rc::new(virtual_fs));

        assert!(!detector.is_javascript_file(Path::new("do-not-exists.js")));
        assert!(!detector.is_javascript_file(Path::new("do-not-exists.jsx")));
        assert!(!detector.is_javascript_file(Path::new("do-not-exists.cjs")));
        assert!(!detector.is_javascript_file(Path::new("do-not-exists.mjs")));
        assert!(!detector.is_javascript_file(Path::new("src/lib.rs")));
        assert!(!detector.is_javascript_file(Path::new("src")));

        assert_eq!(detector.detect_language(Path::new("do-not-exists.js")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.jsx")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.cjs")), None);
        assert_eq!(detector.detect_language(Path::new("do-not-exists.mjs")), None);
        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
