use crate::javascript_language;
use ring_core_content::{DetectLanguage, Language};
use ring_core_fs::PathAdaptator;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use tracing::{instrument, trace};

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
    use mockall::mock;
    use ring_core_fs::PathAdaptator;

    mock! {
        TestAdaptator {}

        impl PathAdaptator for TestAdaptator {
            fn is_supported(&self, path: &Path) -> bool;
            fn is_dir(&self, path: &Path) -> bool;
            fn is_file(&self, path: &Path) -> bool;
        }
    }

    #[test]
    fn it_should_detect_javascript_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .return_const(true);

        let detector = JavascriptFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("assets/test")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.js")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.jsx")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.cjs")), Some(javascript_language()));
        assert_eq!(detector.detect_language(Path::new("assets/test.mjs")), Some(javascript_language()));
    }

    #[test]
    fn it_should_not_detect_javascript_language() {
        let mut path_adaptator = MockTestAdaptator::new();
        path_adaptator.expect_is_file()
            .return_const(false);

        let detector = JavascriptFileDetector::new(Rc::new(path_adaptator));

        assert_eq!(detector.detect_language(Path::new("src/lib.rs")), None);
        assert_eq!(detector.detect_language(Path::new("src")), None);
    }
}
