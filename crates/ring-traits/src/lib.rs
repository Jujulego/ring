mod detector;
mod manifest;
mod module;
mod project;
mod scope;
mod tagged;

pub use detector::{DetectAs, Detector};
pub use manifest::Manifest;
pub use module::Module;
pub use project::{Project, ProjectDetector};
pub use scope::{Scope, ScopeDetector};
pub use tagged::{Tagged, TaggedDetector};
