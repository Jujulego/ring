use crate::units::{Identifier, Unit};
use std::path::Path;
use std::rc::Rc;

type FnIdentifier = dyn Fn(&Path) -> anyhow::Result<Option<Rc<dyn Unit>>>;

#[derive(Default)]
pub struct CombinedIdentifier {
    fns: Vec<Box<FnIdentifier>>,
}

impl CombinedIdentifier {
    pub fn new() -> CombinedIdentifier {
        CombinedIdentifier { fns: Vec::new() }
    }

    pub fn add<U: Unit + 'static, I: Identifier<Unit = Rc<U>> + 'static>(&mut self, identifier: I) {
        self.fns.push(Box::new(move |path| identifier.identify_unit(path)
            .map(|opt| opt
                .map(|unit| unit as Rc<dyn Unit>)
            )
        ));
    }
}

impl Identifier for CombinedIdentifier {
    type Unit = Rc<dyn Unit>;

    fn identify_unit(&self, path: &Path) -> anyhow::Result<Option<Rc<dyn Unit>>> {
        for f in &self.fns {
            if let Some(unit) = f(path)? {
                return Ok(Some(unit));
            }
        }

        Ok(None)
    }
}
