use crate::javascript_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::traits::{FilesystemProtocol, LocationMetadata};
use ring_core_fs::Filesystem;
use std::ffi::OsStr;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::instrument;

#[derive(Clone)]
pub struct JavascriptFileDetector {
    filesystem: Rc<Filesystem>,
}

impl JavascriptFileDetector {
    /// Creates a new instance of JavascriptFileDetector
    #[inline]
    pub fn new(filesystem: Rc<Filesystem>) -> Self {
        Self {
            filesystem,
        }
    }

    /// Checks if given path is a javascript file
    #[inline]
    pub fn is_javascript_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self._is_javascript_file(path.as_ref())
    }

    fn _is_javascript_file(&self, path: &Path) -> bool {
        if !self.filesystem.is_file(path) {
            return false;
        }

        if matches!(path.extension().and_then(OsStr::to_str), Some("js") | Some("jsx") | Some("cjs") | Some("mjs")) {
            return true;
        }

        // Shebangs
        let Ok(mut file) = self.filesystem.locate_path(path) else { return false };

        let reader = BufReader::new(file.read());
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
    use ring_core_fs::protocols::MemoryProtocol;

    #[test]
    fn it_should_detect_javascript_language_using_extensions() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("test.js", "")
                .with_file("test.jsx", "")
                .with_file("test.cjs", "")
                .with_file("test.mjs", "")
        );

        let detector = JavascriptFileDetector::new(Rc::new(filesystem));

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
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("test", "#!/usr/bin/env node")
        );

        let detector = JavascriptFileDetector::new(Rc::new(filesystem));

        assert!(detector.is_javascript_file(Path::new("test")));

        assert_eq!(detector.detect_language(Path::new("test")), Some(javascript_language()));
    }

    #[test]
    fn it_should_not_detect_javascript_language() {
        let filesystem = Filesystem::memory(
            MemoryProtocol::new()
                .with_file("src/lib.rs", "")
        );

        let detector = JavascriptFileDetector::new(Rc::new(filesystem));

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
