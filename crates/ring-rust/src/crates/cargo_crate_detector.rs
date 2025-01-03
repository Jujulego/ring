use crate::{CargoCrate, CargoManifest};
use anyhow::{anyhow, Context};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use tracing::{info, instrument, trace};
use ring_core::{CodeUnit, CodeUnitDetector};

#[derive(Clone, Debug, Default)]
pub struct CargoCrateDetector {
    crates: RefCell<HashMap<PathBuf, Option<Rc<CargoCrate>>>>
}

impl CargoCrateDetector {
    pub fn new() -> CargoCrateDetector {
        Default::default()
    }

    pub fn load_manifest(&self, path: &Path) -> anyhow::Result<Option<CargoManifest>> {
        let manifest_path = path.join("Cargo.toml");

        trace!("read file {}", manifest_path.display());
        match File::open(&manifest_path) {
            Ok(ref mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).context(format!("Unable to read {}", manifest_path.display()))?;

                Some(contents.parse()).transpose()
            }
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => Err(anyhow!(err).context(format!("Unable to access {}", manifest_path.display()))),
        }
    }

    fn _detect_crate(&self, path: &Path) -> anyhow::Result<Option<CargoCrate>> {
        trace!("touch {}", path.display());

        match path.metadata() {
            Ok(metadate) if metadate.is_dir() => {
                if let Some(manifest) = self.load_manifest(path)? {
                    Ok(Some(CargoCrate::new(path.to_path_buf(), manifest)))
                } else {
                    Ok(None)
                }
            }
            Ok(_) => Ok(None),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
            Err(err) => {
                Err(anyhow!(err).context(format!("Unable to access {}", path.display())))
            }
        }
    }

    #[instrument(name = "rust.detect-crate", skip(self, path))]
    pub fn detect_crate<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<Option<Rc<CargoCrate>>> {
        let path = path.as_ref();

        if let Some(crt) = self.crates.borrow().get(path) {
            return Ok(crt.clone())
        }

        let crt = self._detect_crate(path)?
            .map(Rc::new);

        if let Some(crt) = &crt {
            if let Some(name) = crt.name() {
                info!("recognized {} as a rust crate '{}'", path.display(), name);
            } else {
                info!("recognized {} as a rust crate", path.display());
            }

            self.crates.borrow_mut().insert(path.to_path_buf(), Some(crt.clone()));
        } else {
            self.crates.borrow_mut().insert(path.to_path_buf(), None);
        }

        Ok(crt)
    }
}

impl CodeUnitDetector for CargoCrateDetector {
    fn name(&self) -> &str {
        "rust:crate"
    }
    
    fn detect(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn CodeUnit>>> {
        self.detect_crate(path)
            .map(|opt| opt.map(|pkg| pkg as Rc<dyn CodeUnit>))
    }
}