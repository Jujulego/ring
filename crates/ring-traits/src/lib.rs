mod detect;
mod manifest;
mod module;
mod project;
mod scope;

use std::rc::Rc;
use ring_utils::Tagged;
pub use detect::{DetectAs, Detect};
pub use manifest::Manifest;
pub use module::Module;
pub use project::{Project, ProjectDetector, ProjectIterator};
pub use scope::{Scope, ScopeDetector};

pub type TaggedDetector = dyn DetectAs<Rc<dyn Tagged>>;
