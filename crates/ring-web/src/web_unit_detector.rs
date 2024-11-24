use std::path::Path;
use std::rc::Rc;
use ring_code_unit::{CodeUnit, CodeUnitDetector};
use crate::{ScriptFile, WebLanguage};

////////////////////////////////////////////////////////////////////////////////
// Web Unit Detector
////////////////////////////////////////////////////////////////////////////////

pub struct WebUnitDetector {}

impl WebUnitDetector {
    fn _detect_language(&self, path: &Path) -> Option<WebLanguage> {
        if !path.is_file() {
            return None
        }

        match path.extension()?.to_str()? {
            "js" | "jsx" | "cjs" | "mjs" => Some(WebLanguage::JavaScript),
            "ts" | "tsx" | "cts" | "mts" => Some(WebLanguage::TypeScript),
            _ => None
        }
    }

    pub fn detect_script<P: AsRef<Path>>(&self, path: P) -> Option<ScriptFile> {
        let path = path.as_ref();

        self._detect_language(path)
            .map(|language| ScriptFile::new(path.to_path_buf(), language))
    }
}

impl CodeUnitDetector for WebUnitDetector {
    fn detect<P: AsRef<Path>>(&self, path: &P) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        Ok(self.detect_script(path)
            .map(|cu| Rc::new(cu) as Rc<dyn CodeUnit>)
        )
    }
}