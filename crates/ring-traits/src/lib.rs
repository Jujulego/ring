mod detect;
mod manifest;
mod module;
mod project;
mod scope;
mod tagged;

pub use detect::{DetectAs, Detect};
pub use manifest::Manifest;
pub use module::Module;
pub use project::{Project, ProjectDetector, ProjectIterator};
pub use scope::{Scope, ScopeDetector};
pub use tagged::{Tagged, TaggedDetector};
