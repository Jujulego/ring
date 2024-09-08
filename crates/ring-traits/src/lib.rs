mod detect;
mod manifest;
mod module;
mod project;
mod scope;

use std::rc::Rc;
pub use detect::{DetectAs, Detect};
pub use manifest::Manifest;
pub use module::Module;
pub use project::Project;
pub use scope::Scope;

// Aliases
pub type DependencyIterator<'a> = dyn Iterator<Item = ring_utils::Dependency> + 'a;
pub type ProjectDetector = dyn DetectAs<Rc<dyn Project>>;
pub type ProjectIterator<'a> = dyn Iterator<Item = anyhow::Result<Rc<dyn Project>>> + 'a;
pub type ScopeDetector = dyn DetectAs<Rc<dyn Scope>>;
pub type TaggedDetector = dyn DetectAs<Rc<dyn ring_utils::Tagged>>;
